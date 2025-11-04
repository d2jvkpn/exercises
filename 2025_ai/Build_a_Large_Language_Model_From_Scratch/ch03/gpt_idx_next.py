#!/usr/bin/env python3
import torch

####
batch = 10
vocab_size = 10000

logits = torch.randn(batch, vocab_size)
targets = torch.randn(batch, vocab_size)

temperature = 0.2
top_k = 3

####
idx_next = torch.argmax(logits, dim=-1, keepdim=True)

loss = torch.nn.functional.cross_entropy(
    logits.flatten(0, 1),
    targets.flatten(0, 1),
)

loss = torch.tensor(10.7940)
perplexity = torch.exp(loss)

####
top_logits = logits.sort(descending=True).values[:, top_k]
new_logits = torch.where(logits < top_logits.unsqueeze(-1), -torch.inf, logits)

probas = torch.softmax(new_logits / temperature, dim=-1)
idx_next = torch.multinomial(probas, num_samples=1)

####
top_logits, _ = torch.topk(logits, top_k)
min_val = top_logits[:, [-1]]
new_logits = torch.where(logits < min_val, -torch.inf, logits)

probs = torch.softmax(new_logits / temperature, dim=-1)
idx_next = torch.multinomial(probs, num_samples=1)
