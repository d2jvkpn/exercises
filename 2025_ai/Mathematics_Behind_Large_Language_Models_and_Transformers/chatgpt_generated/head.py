#!/usr/bin/env python3
import math

import torch
import torch.nn as nn
import torch.nn.functional as F

class Head(nn.Module):
    """
    单头自注意力（Self-Attention Head）
    x: (B, T, d_model) -> out: (B, T, d_head)
    """
    def __init__(self, d_model: int, d_head: int, block_size: int,
                 dropout: float = 0.0, bias: bool = False, causal: bool = True):
        super().__init__()
        self.causal = causal
        self.scale = 1.0 / math.sqrt(d_head)

        # Q, K, V 线性映射
        self.q_proj = nn.Linear(d_model, d_head, bias=bias)
        self.k_proj = nn.Linear(d_model, d_head, bias=bias)
        self.v_proj = nn.Linear(d_model, d_head, bias=bias)

        # 因果下三角 mask（最大支持 block_size）
        self.register_buffer(
            "tril",
            torch.tril(torch.ones(block_size, block_size)),
            persistent=False
        )

        self.dropout = nn.Dropout(dropout)

    def forward(self, x: torch.Tensor, attn_mask: torch.Tensor | None = None,
                need_weights: bool = False):
        """
        x: (B, T, d_model)
        attn_mask: 可选的 (B, T) bool/0-1 掩码（True/1=可见；False/0=屏蔽，用于 padding）
        need_weights: 是否返回注意力权重 (B, T, T)
        """
        B, T, _ = x.shape

        # 1) 线性映射
        q = self.q_proj(x)  # (B, T, d_head)
        k = self.k_proj(x)  # (B, T, d_head)
        v = self.v_proj(x)  # (B, T, d_head)

        # 2) 注意力分数 (B, T, T)
        scores = (q @ k.transpose(-2, -1)) * self.scale

        # 3) 因果 mask
        if self.causal:
            scores = scores.masked_fill(self.tril[:T, :T] == 0, float("-inf"))

        # 4) 外部 mask（屏蔽被 pad 的 key 位置）
        if attn_mask is not None:
            if attn_mask.dtype != torch.bool:
                attn_mask = attn_mask.bool()
            key_mask = attn_mask.unsqueeze(1)  # (B, 1, T)，对所有 query 位置复用
            scores = scores.masked_fill(~key_mask, float("-inf"))

        # 5) Softmax + Dropout
        attn = F.softmax(scores, dim=-1)      # (B, T, T)
        attn = self.dropout(attn)

        # 6) 加权求和得到输出
        out = attn @ v                         # (B, T, d_head)

        return (out, attn) if need_weights else out

if __name__ == "__main__":
    torch.manual_seed(0)
    B, T, d_model, d_head = 2, 5, 32, 8
    head = Head(d_model, d_head, block_size=128, dropout=0.1, causal=True)

    x = torch.randn(B, T, d_model)
    out, attn = head(x, need_weights=True)
    print(out.shape, attn.shape)  # torch.Size([2, 5, 8]) torch.Size([2, 5, 5])
