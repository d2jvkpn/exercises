#!/usr/bin/env python3

from .head import Head

import torch
import torch.nn as nn

class MultiHeadAttention(nn.Module):
    """
    由多个 Head 组成的 Multi-Head Self-Attention
    x: (B, T, d_model) -> out: (B, T, d_model)
    """
    def __init__(self, d_model: int, n_heads: int, block_size: int,
                 dropout: float = 0.0, bias: bool = False, causal: bool = True):
        super().__init__()
        assert d_model % n_heads == 0, "d_model 必须能被 n_heads 整除"
        d_head = d_model // n_heads

        # 复用你已有的 Head 类
        self.heads = nn.ModuleList([
            Head(d_model, d_head, block_size, dropout=dropout, bias=bias, causal=causal)
            for _ in range(n_heads)
        ])

        self.out_proj = nn.Linear(d_model, d_model, bias=bias)
        self.out_dropout = nn.Dropout(dropout)
        self.n_heads = n_heads
        self.d_model = d_model
        self.d_head = d_head

    @torch.no_grad()
    def _stack_attn(self, attn_list):
        # list[(B,T,T)] -> (B, n_heads, T, T) 再均值
        return torch.stack(attn_list, dim=1).mean(dim=1)

    def forward(self, x: torch.Tensor, attn_mask: torch.Tensor | None = None,
                need_weights: bool = False, average_attn_weights: bool = True):
        """
        attn_mask: (B, T) 的 bool/0-1 掩码（True/1=可见，False/0=屏蔽）
        need_weights: 是否返回注意力权重
        average_attn_weights: True 返回各头平均后的 (B,T,T)，False 返回每头的 list
        """
        outs = []
        attn_list = [] if need_weights else None

        for h in self.heads:
            if need_weights:
                o, a = h(x, attn_mask=attn_mask, need_weights=True)  # o:(B,T,d_head), a:(B,T,T)
                outs.append(o)
                attn_list.append(a)
            else:
                outs.append(h(x, attn_mask=attn_mask, need_weights=False))

        # 拼接多头 -> (B, T, d_model)
        y = torch.cat(outs, dim=-1)
        # 输出投影
        y = self.out_dropout(self.out_proj(y))

        if need_weights:
            if average_attn_weights:
                return y, self._stack_attn(attn_list)   # (B,T,T)
            else:
                return y, attn_list                     # list of (B,T,T)
        return y

if __name__ == "__main__":
    torch.manual_seed(0)
    B, T, d_model, n_heads = 2, 5, 32, 4
    mha = MultiHeadAttention(d_model, n_heads, block_size=128, dropout=0.1, causal=True)
    x = torch.randn(B, T, d_model)

    y, attn = mha(x, need_weights=True)   # attn: (B, T, T)
    print("y:", y.shape, " attn:", attn.shape)  # y: torch.Size([2, 5, 32])  attn: torch.Size([2, 5, 5])
