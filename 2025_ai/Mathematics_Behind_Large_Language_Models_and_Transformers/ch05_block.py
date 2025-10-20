#!/usr/bin/env python3
import numpy as np
np.random.seed(0)

import torch.nn as nn

# ====== 超参数 ======
B = 10                            # batch size
T = 42                            # seq_len
d_model = 768                     # 隐藏维度, embedding
num_heads = 12                    # 头数
head_size = d_model // num_heads  # 单头维度 d_k = d_v
assert num_heads * head_size == d_model
dropout_p = 0.2

def dropout(x, dropout_p, training=True):
    if not training or dropout_p == 0.0:
        return x

    keep_prob = 1 - dropout_p
    mask = (np.random.rand(*x.shape) < keep_prob).astype(np.float32)
    return (x * mask) / keep_prob

print(O_projected)

#### 0. embedding

#### 1. Layer Norm 1

#### 2. Multi-Head Attention

#### 3. Feed Forward Network (FFN)
##### 3.1 Linear
##### 3.2 GELU or ReLU
##### 3.3 Linear
##### 3.4 Dropout

#### 4. Layer Norm 2


####
class MultiHeadAttention(nn.Module):
    """ multiple heads of self-attention in parallel """

    def __init__(self,
        n_embd: int, num_heads: int, head_size: int, block_size: int, dropout: float,
    ) -> None:
        super().__init__()

        self.heads = nn.ModuleList([
            Head(n_embd, head_size, block_size, dropout)
            for _ in range(num_heads)
        ])

        self.projection = nn.Linear(head_size * num_heads, n_embd)
        self.dropout = nn.Dropout(dropout)

    def forward(self, x: torch.Tensor) -> torch.Tensor:
        out = torch.cat([h(x) for h in self.heads], dim=-1)
        out = self.dropout(self.projection(out))
        return out


class FeedForward(nn.Module):
    """ a simple linear layer followed by a non-linearity """

    def __init__(self, n_embd: int, dropout: float) -> None:
        super().__init__()

        self.net = nn.Sequential(
            nn.Linear(n_embd, 4 * n_embd),
            nn.ReLU(),
            nn.Linear(4 * n_embd, n_embd),
            nn.Dropout(dropout),
        )

    def forward(self, x: torch.Tensor) -> torch.Tensor:
        return self.net(x)


class Block(nn.Module):
    """ Transformer block: communication followed by computation """

    def __init__(self, n_embd: int, n_head: int, block_size: int, dropout: float) -> None:
        super().__init__()

        head_size = n_embd // n_head
        err_msg = f"n_embd {n_embd} must be divisible by n_head {n_head}"
        assert head_size * n_head == n_embd, err_msg

        self.layer_norm_1 = nn.LayerNorm(n_embd)

        self.self_attention = MultiHeadAttention(
            n_embd=n_embd,
            num_heads=n_head,
            head_size=head_size,
            block_size=block_size,
            dropout=dropout,
        )

        self.feed_forward = FeedForward(n_embd, dropout)

        self.layer_norm_2 = nn.LayerNorm(n_embd)

    def forward(self, x: torch.Tensor) -> torch.Tensor:
        x = x + self.self_attention(self.layer_norm_1(x))
        x = x + self.feed_forward(self.layer_norm_2(x))
        return x
