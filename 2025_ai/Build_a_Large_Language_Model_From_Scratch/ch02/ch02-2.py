#!/usr/bin/env python3
import torch
torch.manual_seed(123)

####
vocab_size = 8 # index range: (0, 7)
output_dim = 3

embedding_layer = torch.nn.Embedding(vocab_size, output_dim)
print(embedding_layer.weight)

input_ids = torch.tensor([2, 3, 5, 1])
embed = embedding_layer(input_ids)
print(embed)

assert(embed.shape[0] == len(input_ids))
assert(embed.shape[1] == embedding_layer.embedding_dim)
