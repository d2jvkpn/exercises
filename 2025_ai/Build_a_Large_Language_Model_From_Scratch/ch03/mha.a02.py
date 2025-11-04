#!/usr/bin/env python3
import torch
from torch import nn

#### 1. large language models
n_vocab = 50257
n_embd, context_length, n_heads = 768, 1024, 12 # n_embd: embedding size
drop_rate = 0.1
d_out = n_embd

assert(d_out % n_heads == 0)
d_head = d_out // n_heads

mask_bool = torch.triu(torch.ones(context_length, context_length), diagonal=1).bool()
dropout = nn.Dropout(drop_rate)
out_proj = torch.nn.Linear(d_out, d_out)

w_queries = nn.Linear(n_embd, d_out, bias=False)
w_keys = nn.Linear(n_embd, d_out, bias=False)
w_values = nn.Linear(n_embd, d_out, bias=False)

#### 2. inputs, shape=(B, T, n_embd)
B, T = 10, 42
x = torch.randn(B, T, n_embd)
assert(T <= context_length)

#### 3. multi-head attention
# (B, T, n_embd) @ (n_embd, d_out) => (B, T, n_embd)
queries = w_queries(x) # x @ Q
keys = w_keys(x)       # x @ K
values = w_values(x)   # x @ V

# (B, T, n_embd) => (B, T, n_heads, d_head)
queries = queries.view(B, T, n_heads, d_head)
keys = keys.view(B, T, n_heads, d_head)
values = values.view(B, T, n_heads, d_head)

# (B, T, n_heads, d_head) => (V, n_heads, T, d_head)
queries = queries.transpose(1, 2)
keys = keys.transpose(1, 2)
values = values.transpose(1, 2)

# (B, n_heads, T, d_head) @ (B, n_heads, d_head, T) => (B, n_heads, T, T)
attn_scores = queries @ keys.transpose(2, 3)
attn_scores.masked_fill_(mask_bool[:T, :T], -torch.inf)

attn_weights = torch.softmax(attn_scores / keys.shape[-1]**0.5, dim=-1)
attn_weights = dropout(attn_weights)

# (B, n_heads, T, T) @ (B, n_heads, T, d_head) => (B, n_heads, T, d_head)
context_vec = attn_weights @ values

# (B, n_heads, T, d_head) => (B, T, n_heads, d_head)
context_vec = context_vec.transpose(1, 2)

# (B, T, n_heads, d_head) => (B, T, d_out)
context_vec = context_vec.contiguous().view(B, T, d_out)

#### 4. projection
context_vec = out_proj(context_vec)
