#!/usr/bin/env python3
import math

import torch
import torch.nn as nn
import torch.nn.functional as F

class FastMultiHeadAttention(nn.Module):
    """
    高效 Multi-Head Self-Attention：
      - 一次线性层同时得到 Q/K/V（更少 kernel 调用）
      - 支持因果 mask（自回归）
      - 支持外部 padding mask
      - 可选返回注意力权重（需要时走手写路径）
      - 当 need_weights=False 且无需外部 mask 时，优先使用 SDPA（可能启用 Flash/Math kernels）
    形状：
      x: (B, T, d_model)  ->  out: (B, T, d_model)
    """
    def __init__(self, d_model: int, n_heads: int, block_size: int,
                 dropout: float = 0.0, bias: bool = False, causal: bool = True,
                 try_sdpa: bool = True):
        super().__init__()
        assert d_model % n_heads == 0, "d_model 必须能被 n_heads 整除"
        self.d_model = d_model
        self.n_heads = n_heads
        self.d_head  = d_model // n_heads
        self.causal = causal
        self.try_sdpa = try_sdpa

        # 一次性投影出 Q/K/V
        self.in_proj  = nn.Linear(d_model, 3 * d_model, bias=bias)
        # 输出融合
        self.out_proj = nn.Linear(d_model, d_model, bias=bias)

        self.attn_dropout = nn.Dropout(dropout)
        self.resid_dropout = nn.Dropout(dropout)

        # 因果下三角 mask buffer
        self.register_buffer(
            "tril",
            torch.tril(torch.ones(block_size, block_size)),
            persistent=False
        )

    def _split_heads(self, t: torch.Tensor):
        # (B, T, d_model) -> (B, nH, T, dH)
        B, T, _ = t.shape
        return t.view(B, T, self.n_heads, self.d_head).transpose(1, 2)

    def _merge_heads(self, t: torch.Tensor):
        # (B, nH, T, dH) -> (B, T, d_model)
        B, nH, T, dH = t.shape
        return t.transpose(1, 2).contiguous().view(B, T, nH * dH)

    def forward(self, x: torch.Tensor,
                attn_mask: torch.Tensor | None = None,     # (B, T) True/1=valid
                need_weights: bool = False,
                average_attn_weights: bool = True):
        """
        当 need_weights=True 或提供 attn_mask 时，使用手写路径（支持更复杂的 mask，并可返回权重）。
        当 need_weights=False 且 attn_mask=None 且 try_sdpa=True，则优先走 SDPA 快速路径。
        """
        B, T, _ = x.shape

        # 1) 一次性 QKV
        qkv = self.in_proj(x)                            # (B, T, 3*d_model)
        q, k, v = qkv.split(self.d_model, dim=-1)        # 各 (B, T, d_model)

        # 2) reshape 为多头
        q = self._split_heads(q)                         # (B, nH, T, dH)
        k = self._split_heads(k)
        v = self._split_heads(v)

        # ---- SDPA 快速路径（仅在安全场景下启用）----
        can_use_sdpa = (
            self.try_sdpa and
            (attn_mask is None) and
            (not need_weights)
        )
        if can_use_sdpa:
            # 合并 batch 与 head 维度以适配 SDPA
            BnH = B * self.n_heads
            q_ = q.reshape(BnH, T, self.d_head)
            k_ = k.reshape(BnH, T, self.d_head)
            v_ = v.reshape(BnH, T, self.d_head)

            # PyTorch 会根据 dtype / 设备选择 Flash/Math kernels
            y = F.scaled_dot_product_attention(
                q_, k_, v_,
                attn_mask=None,
                dropout_p=self.attn_dropout.p if self.training else 0.0,
                is_causal=self.causal
            )                                            # (BnH, T, dH)
            y = y.view(B, self.n_heads, T, self.d_head)  # (B, nH, T, dH)
            y = self._merge_heads(y)                     # (B, T, d_model)
            y = self.resid_dropout(self.out_proj(y))     # (B, T, d_model)
            return y

        # ---- 手写路径（支持外部 mask / 返回权重）----
        # 注意力分数: (B, nH, T, dH) @ (B, nH, dH, T) -> (B, nH, T, T)
        scores = (q @ k.transpose(-2, -1)) / math.sqrt(self.d_head)

        # 因果 mask
        if self.causal:
            causal = self.tril[:T, :T].unsqueeze(0).unsqueeze(0)  # (1,1,T,T)
            scores = scores.masked_fill(causal == 0, float("-inf"))

        # 外部 padding mask（屏蔽无效 key 位置）
        if attn_mask is not None:
            if attn_mask.dtype != torch.bool:
                attn_mask = attn_mask.bool()
            key_mask = attn_mask.view(B, 1, 1, T)  # (B,1,1,T)
            scores = scores.masked_fill(~key_mask, float("-inf"))

        # Softmax -> 注意力
        attn = F.softmax(scores, dim=-1)                 # (B, nH, T, T)
        attn = self.attn_dropout(attn)

        # 加权求和
        y = attn @ v                                     # (B, nH, T, dH)

        # 合并多头 + 输出投影
        y = self._merge_heads(y)                         # (B, T, d_model)
        y = self.resid_dropout(self.out_proj(y))         # (B, T, d_model)

        if need_weights:
            if average_attn_weights:
                # 平均各头: (B, T, T)
                attn_mean = attn.mean(dim=1)
                return y, attn_mean
            else:
                # 返回每头: (B, nH, T, T)
                return y, attn
        return y

if __name__ == "__main__":
    torch.manual_seed(0)
    B, T, d_model, n_heads = 2, 6, 64, 8
    mha = FastMultiHeadAttention(d_model, n_heads, block_size=128,
                                 dropout=0.1, causal=True, try_sdpa=True)

    x = torch.randn(B, T, d_model)
    # 快速路径（SDPA）：
    y1 = mha(x)  # (B, T, d_model)

    # 带 padding mask（自动回退到手写路径）：
    pad_mask = torch.tensor([[1,1,1,1,0,0],[1,1,1,1,1,1]], dtype=torch.bool)
    y2, attn = mha(x, attn_mask=pad_mask, need_weights=True)  # y2:(B,T,d_model), attn:(B,T,T)

    print(y1.shape, y2.shape, attn.shape)  # torch.Size([2, 6, 64]) torch.Size([2, 6, 64]) torch.Size([2, 6, 6])
