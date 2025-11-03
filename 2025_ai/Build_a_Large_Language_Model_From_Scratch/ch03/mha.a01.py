#!/usr/bin/env python3
import torch

mat = torch.tensor([[
  [
    [0.2745, 0.6584, 0.2775, 0.8573],
    [0.8993, 0.0390, 0.9268, 0.7388],
    [0.7179, 0.7058, 0.9156, 0.4340]
  ],
  [
    [0.0772, 0.3565, 0.1479, 0.5331],
    [0.4066, 0.2318, 0.4545, 0.9737],
    [0.4606, 0.5159, 0.4220, 0.5786],
  ],
]])

####
(b, num_heads, num_tokens, head_dim) = mat.shape
print(f"b={b}, num_heads={num_heads}, num_tokens={num_tokens}, head_dim={head_dim}")
# b=1, num_heads=2, num_tokens=3, head_dim=4
d_out = num_heads * head_dim

out_proj = torch.nn.Linear(d_out, d_out)

print((mat @ mat.transpose(2, 3)).shape) # (1, 2, 3, 3)

####
first_head = mat[0, 0, :, :]
first_res = first_head @ first_head.T
print("First head:\n", first_res)

second_head =  mat[0, 1, :, :]
second_res = second_head @ second_head.T
print("\nSecond head:\n", second_res)

####
heads = torch.stack([first_head, second_head])
context_vec = heads.contiguous().view(b, num_tokens, d_out)

#####
# (b, num_tokens, d_in) @ (d_in, d_out) => (b, num_tokens, d_out)
# query = x @ Q

# (b, num_tokens, d_out) => (b, num_tokens, num_heads, head_dim)
# query = query.view(b, num_tokens, n_heads, head_dim)

# (b, num_tokens, num_heads, head_dim) => (b, num_heads, num_tokens, head_dim)
query = mat.transpose(1, 2)
keys = mat.transpose(1, 2)
values = mat.transpose(1, 2)

# (b, num_tokens, num_heads, head_dim) @ (b, num_tokens, head_dim, num_heads)
# => (b, num_tokens, num_heads, num_heads)
attn_scores = query @ keys.transpose(2, 3) 

# (b, num_tokens, num_heads, num_heads) @ (b, num_tokens, num_heads, head_dim)
# => (b, num_tokens, num_heads, head_dim)
context_vec1 = (attn_scores @ values)

# (b, num_tokens, num_heads, head_dim) => (b, num_heads, num_tokens, head_dim)
context_vec2 = context_vec1.transpose(1, 2)

# (b, num_heads, num_tokens, head_dim) => (b, num_tokens, d_out)
context_vec3 = context_vec2.contiguous().view(b, num_tokens, d_out)

context_vec4 = context_vec3 @ out_proj
