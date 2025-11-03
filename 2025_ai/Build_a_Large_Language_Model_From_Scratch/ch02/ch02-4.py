#!/usr/bin/env python3
import torch
import torch.nn.functional as F

inputs = torch.tensor([
    [0.43, 0.15, 0.89], # Your (x^1)
    [0.55, 0.87, 0.66], # journey (x^2)
    [0.57, 0.85, 0.64], # starts (x^3)
    [0.22, 0.58, 0.33], # with (x^4)
    [0.77, 0.25, 0.10], # one (x^5)
    [0.05, 0.80, 0.55], # step (x^6)
])

####
query = inputs[1] # 第二个输入作为查询

attn_scores = (query.unsqueeze(0) * inputs).sum(1)
#attn_scores = torch.empty(inputs.shape[0])
#for i, x_i in enumerate(inputs):
#    attn_scores[i] = torch.dot(x_i, query)

print(f"Attention scores: {attn_scores}")

wts_1 = attn_scores / attn_scores.sum()
print(f"Attention weights: {wts_1}, Sum={wts_1.sum().round()}")

def softmax_naive(x):
    return torch.exp(x) / torch.exp(x).sum(dim=0)

wts_2 = softmax_naive(attn_scores)

wts_3 = F.softmax(attn_scores, dim=0) # torch.softmax

####
attn_scores = inputs @ inputs.T
attn_weights = torch.softmax(attn_scores, dim=-1)

print("All row sums:", attn_weights.sum(dim=-1))

####
all_context_vecs = attn_weights @ inputs
print(all_context_vecs)
