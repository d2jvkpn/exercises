#!/usr/bin/env python3
import math

import torch
import torch.nn as nn
import torch.nn.functional as F

class MultiHeadAttention(nn.Module):
    """
    标准 Multi-Head Self-Attention（单层、自注意力版）
    - 一次性 QKV 线性投影（更高效）
    - 因果 Mask（自回归）
    - 可选外部 mask（如 padding mask）
    形状约定:
      x: (B, T, d_model)
      返回: (B, T, d_model)
    """
    def __init__(
        self,
        d_model: int,
        n_heads: int,
        block_size: int,
        dropout: float = 0.0,
        bias: bool = False,
        causal: bool = True,
    ):
        super().__init__()
        assert d_model % n_heads == 0, "d_model 必须能被 n_heads 整除"

        self.d_model = d_model
        self.n_heads = n_heads
        self.d_head = d_model // n_heads

        self.causal = causal

        # 一次性投影出 Q、K、V（3 * d_model）
        self.in_proj = nn.Linear(d_model, 3 * d_model, bias=bias)
        # 输出融合
        self.out_proj = nn.Linear(d_model, d_model, bias=bias)

        self.attn_dropout = nn.Dropout(dropout)
        self.resid_dropout = nn.Dropout(dropout)

        # 因果 mask（下三角），最大支持 block_size
        self.register_buffer(
            "tril",
            torch.tril(torch.ones(block_size, block_size)),
            persistent=False,  # 训练时不会随 ckpt 一起保存很大 tensor
        )

    def forward(self, x: torch.Tensor, attn_mask: torch.Tensor | None = None):
        """
        x: (B, T, d_model)
        attn_mask: 可选 (B, T) 的 0/1 或 bool；为 False/0 的位置将被屏蔽（如 padding）。
        """
        B, T, _ = x.shape

        # 1) 一次性线性: (B, T, 3*d_model) -> 切分为 Q,K,V
        qkv = self.in_proj(x)  # (B, T, 3*d_model)
        q, k, v = qkv.split(self.d_model, dim=-1)  # 各 (B, T, d_model)

        # 2) 变形为多头: (B, T, n_heads, d_head) -> (B, n_heads, T, d_head)
        def split_heads(t):
            return t.view(B, T, self.n_heads, self.d_head).transpose(1, 2)

        q = split_heads(q)
        k = split_heads(k)
        v = split_heads(v)

        # 3) 注意力分数: (B, n_heads, T, d_head) @ (B, n_heads, d_head, T) -> (B, n_heads, T, T)
        attn_scores = (q @ k.transpose(-2, -1)) / math.sqrt(self.d_head)

        # 4) 因果 Mask（防未来信息泄漏）
        if self.causal:
            # tril[:T, :T] 形状 (T, T) -> 扩展到 (1, 1, T, T)
            causal_mask = self.tril[:T, :T].unsqueeze(0).unsqueeze(0)  # 1×1×T×T
            attn_scores = attn_scores.masked_fill(causal_mask == 0, float("-inf"))

        # 5) 可选外部 mask（如 padding mask）
        #    形状 (B, T) -> (B, 1, 1, T)，屏蔽被 pad 的 key 位置
        if attn_mask is not None:
            # 期望 True/1 表示“可见”，False/0 表示“屏蔽”
            if attn_mask.dtype != torch.bool:
                attn_mask = attn_mask.bool()

            key_mask = attn_mask.unsqueeze(1).unsqueeze(1)  # (B,1,1,T)
            attn_scores = attn_scores.masked_fill(~key_mask, float("-inf"))

        # 6) Softmax -> 注意力权重
        attn = F.softmax(attn_scores, dim=-1)              # (B, n_heads, T, T)
        attn = self.attn_dropout(attn)

        # 7) 加权求和: (B, n_heads, T, T) @ (B, n_heads, T, d_head) -> (B, n_heads, T, d_head)
        y = attn @ v

        # 8) 合并多头: (B, n_heads, T, d_head) -> (B, T, n_heads, d_head) -> (B, T, d_model)
        y = y.transpose(1, 2).contiguous().view(B, T, self.d_model)

        # 9) 输出线性 + dropout
        y = self.out_proj(y)
        y = self.resid_dropout(y)
        return y


if __name__ == "__main__":
    torch.manual_seed(0)
    B, T, d_model, n_heads = 2, 5, 32, 4
    block_size = 128
    dropout = 0.1

    mha = MultiHeadAttention(
        d_model=d_model, n_heads=n_heads,
        block_size=block_size, dropout=dropout, causal=True,
    )

    x = torch.randn(B, T, d_model)

    # 可选：padding mask，1 表示有效 token
    # attn_mask = torch.tensor([[1,1,1,0,0],
    #                           [1,1,1,1,1]], dtype=torch.bool)

    result = mha(x)  # or mha(x, attn_mask=attn_mask)
    print(f"Result: {result.shape}\n{result}")  # -> (2, 5, 32)
