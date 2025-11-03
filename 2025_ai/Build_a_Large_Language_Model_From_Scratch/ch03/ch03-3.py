#!/usr/bin/env python3
import torch
from torch import nn
torch.manual_seed(123)

####
inputs = torch.tensor([
  [0.43, 0.15, 0.89], # Your (x^1)
  [0.55, 0.87, 0.66], # journey (x^2)
  [0.57, 0.85, 0.64], # starts (x^3)
  [0.22, 0.58, 0.33], # with (x^4)
  [0.77, 0.25, 0.10], # one (x^5)
  [0.05, 0.80, 0.55], # step (x^6)
])

d_in = 3
d_out = 2
context_length = 6

assert(d_in == inputs.shape[-1])
assert(context_length >= inputs.shape[0])

####
W_query = torch.nn.Parameter(torch.rand(d_in, d_out), requires_grad=False)
W_key = torch.nn.Parameter(torch.rand(d_in, d_out), requires_grad=False)
W_value = torch.nn.Parameter(torch.rand(d_in, d_out), requires_grad=False)

queries = inputs @ W_query
keys = inputs @ W_key
values = inputs @ W_value

####
attn_scores = queries @ keys.T

mask = torch.triu(torch.ones(context_length, context_length), diagonal=1)
masked = attn_scores.masked_fill(mask.bool(), -torch.inf)
print(masked)

attn_weights = torch.softmax(masked / keys.shape[-1]**0.5, dim=1)
print(attn_weights)
print(attn_weights.sum(dim=-1))


class CausalAttention(nn.Module):
    def __init__(self, d_in, d_out, context_length, dropout, qkv_bias=False):
        super().__init__()
        self.d_out = d_out
        self.W_query = nn.Linear(d_in, d_out, bias=qkv_bias)
        self.W_key = nn.Linear(d_in, d_out, bias=qkv_bias)
        self.W_value = nn.Linear(d_in, d_out, bias=qkv_bias)
        self.dropout = nn.Dropout(dropout)
        triu = torch.triu(torch.ones(context_length, context_length), diagonal=1)
        self.register_buffer('mask', triu)

    def forward(self, x):
        b, num_tokens, d_in = x.shape
        # step1
        queries = self.W_query(x)
        keys = self.W_key(x)
        values = self.W_value(x)

        # step2
        attn_scores = queries @ keys.transpose(1, 2)
        attn_scores.masked_fill_(self.mask.bool()[:num_tokens, :num_tokens], -torch.inf)
        attn_weights = torch.softmax(attn_scores / keys.shape[-1]**0.5, dim=-1)
        attn_weights = self.dropout(attn_weights)

        # step3
        context_vec = attn_weights @ values
        return context_vec

class MultiHeadAttentionWrapper(nn.Module):
    def __init__(self, d_in, d_out, context_length, dropout, num_heads, qkv_bias=False):
        super().__init__()

        self.heads = nn.ModuleList([
            CausalAttention(d_in, d_out, context_length, dropout, qkv_bias)
            for _ in range(num_heads)
        ])

    def forward(self, x):
        return torch.cat([head(x) for head in self.heads], dim=-1)


mha = MultiHeadAttentionWrapper(d_in, d_out, context_length, 0.0, num_heads=4)

batch = torch.stack((inputs, inputs), dim=0)
context_vecs = mha(batch)
