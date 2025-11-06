#!/usr/bin/env python3
import torch
from torch import nn

#### 1. large language models
# n_vocab: vocabulary size of tokenizer
# d_embd: dimensions of embedding, embedding size
# n_context: context length
# n_heads: number of heads
# n_layers: number of layers
# d_head: dimensions of a head
n_vocab = 50257
n_context = 1024
d_embd, n_heads, n_layers = 768, 12, 12
drop_rate = 0.1
d_out = d_embd
assert(d_out % n_heads == 0)
d_head = d_out // n_heads

# casual attention mask
#mask_bool = torch.triu(torch.ones(n_context, n_context), diagonal=1).bool()
mask_bool = torch.triu(torch.ones(n_context, n_context, dtype=torch.bool), diagonal=1)
dropout = nn.Dropout(drop_rate)
out_proj = torch.nn.Linear(d_out, d_out)

W_query = nn.Linear(d_embd, d_out, bias=False)
W_key = nn.Linear(d_embd, d_out, bias=False)
W_value = nn.Linear(d_embd, d_out, bias=False)

#### 2. inputs, shape=(B, T, d_embd)
B, T = 10, 42
x = torch.randn(B, T, d_embd)
assert(T <= n_context)

#### 3. multi-head attention
# 3.1 (B, T, d_embd) @ (d_embd, d_out) => (B, T, d_embd)
queries = W_query(x)  # x @ Q
keys = W_key(x)       # x @ K
values = W_value(x)   # x @ V

# 3.2 (B, T, d_embd) => (B, T, n_heads, d_head)
queries = queries.view(B, T, n_heads, d_head)
keys = keys.view(B, T, n_heads, d_head)
values = values.view(B, T, n_heads, d_head)

# 3.3 (B, T, n_heads, d_head) => (V, n_heads, T, d_head)
queries = queries.transpose(1, 2)
keys = keys.transpose(1, 2)
values = values.transpose(1, 2)

# 3.4 (B, n_heads, T, d_head) @ (B, n_heads, d_head, T) => (B, n_heads, T, T)
attn_scores = queries @ keys.transpose(2, 3)
attn_scores.masked_fill_(mask_bool[:T, :T], -torch.inf)

# 3.5 attn_weights = torch.softmax(attn_scores / keys.shape[-1]**0.5, dim=-1)
attn_weights = torch.softmax(attn_scores / d_head**0.5, dim=-1)
attn_weights = dropout(attn_weights)

# 3.6 (B, n_heads, T, T) @ (B, n_heads, T, d_head) => (B, n_heads, T, d_head)
context_vec = attn_weights @ values
# (B, n_heads, T, d_head) => (B, T, n_heads, d_head)
context_vec = context_vec.transpose(1, 2)

# 3.7 (B, T, n_heads, d_head) => (B, T, d_out)
context_vec = context_vec.contiguous().view(B, T, d_out)

#### 4. projection
context_vec = out_proj(context_vec)
