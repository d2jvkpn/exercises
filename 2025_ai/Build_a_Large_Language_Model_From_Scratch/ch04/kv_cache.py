#!/usr/bin/env python3
import torch
torch.manual_seed(123)
torch.set_printoptions(sci_mode=False)


d_embd = 16
n_heads = 4
d_head = d_embd // n_heads

Q = torch.randn(d_embd, d_embd)
K = torch.randn(d_embd, d_embd)
V = torch.randn(d_embd, d_embd)

#### 1. 
B = 10
T = 42
x_1 = torch.randn(B, T, d_embd)
idx_next = torch.randn(B, 1, d_embd)

# shape=(B, n_heads, T, d_embd)
queries = (x_1 @ Q).view(B, T, n_heads, d_head).transpose(1, 2)
keys_1 = (x_1 @ K).view(B, T, n_heads, d_head).transpose(1, 2)
values_1 = (x_1 @ V).view(B, T, n_heads, d_head).transpose(1, 2)

# shape=(B, n_heads, T, T)
attn_scores_1 = queries @ keys_1.transpose(2, 3)
mask_bool = torch.triu(torch.ones(T, T, dtype=torch.bool), diagonal=1)
attn_scores_1.masked_fill_(mask_bool, -torch.inf)
attn_weights_1 = torch.softmax(attn_scores_1 / d_head**0.5, dim=-1)

attn_out_1 = (attn_weights_1 @ values_1).transpose(1, 2).contiguous().view(B, T, d_embd)

#### 2. no kv-cache
x_2 = torch.concat((x_1, idx_next), dim=1) # shape=(B, T+1, d_embd)

# shape=(B, n_heads, T+1, d_embd)
queries = (x_2 @ Q).view(B, T+1, n_heads, d_head).transpose(1, 2)
keys_2 = (x_2 @ K).view(B, T+1, n_heads, d_head).transpose(1, 2)
values_2 = (x_2 @ V).view(B, T+1, n_heads, d_head).transpose(1, 2)

# shape=(B, n_heads, T+1, T+1)
attn_scores_2 = queries @ keys_2.transpose(2, 3)
mask_bool = torch.triu(torch.ones(T+1, T+1, dtype=torch.bool), diagonal=1)
attn_scores_2.masked_fill_(mask_bool, -torch.inf)
attn_weights_2 = torch.softmax(attn_scores_2 / d_head**0.5, dim=-1)

attn_out_2 = (attn_weights_2 @ values_2).transpose(1, 2).contiguous().view(B, T+1, d_embd)

#### 3. using kv-cache
x_3 = idx_next # shape=(B, 1, d_embd)

# shape=(B, n_heads, 1, d_embd)
queries = (x_3 @ Q).view(B, 1, n_heads, d_head).transpose(1, 2)
keys_3 = (x_3 @ K).view(B, 1, n_heads, d_head).transpose(1, 2)
values_3 = (x_3 @ V).view(B, 1, n_heads, d_head).transpose(1, 2)

# shape=(B, n_heads, T+1, d_embd)
keys_3 = torch.concat((keys_1, keys_3), dim=2)        # using keys_1 as cache
values_3 = torch.concat((values_1, values_3), dim=2)  # using values_1 as cache

# shape=(B, n_heads, 1, T+1)
attn_scores_3 = queries @ keys_3.transpose(2, 3)
#mask_bool = torch.triu(torch.ones(T+1, T+1, dtype=torch.bool), diagonal=1)
#attn_scores.masked_fill_(mask_bool, -torch.inf)
attn_weights_3 = torch.softmax(attn_scores_3 / d_head**0.5, dim=-1)

attn_out_3 = (attn_weights_3 @ values_3).transpose(1, 2).contiguous().view(B, 1, d_embd)

####
delta = attn_out_2[:, -1, :] - attn_out_3[:, 0, :]
print(delta.min(), delta.max())
