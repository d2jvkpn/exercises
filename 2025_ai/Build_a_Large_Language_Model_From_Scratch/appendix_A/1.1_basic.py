#!/usr/bin/env python3
import torch

####
t0 = tensor(1)

t1 = tensor([1, 2, 3])

t2 = tensor([
    [1, 2],
    [3, 4],
])

t3 = tensor([
  [
    [1, 2],
    [3, 4],
  ],
  [
    [5, 6],
    [7, 8],
  ],
])

####
print(t1.dtype)

f1 = torch.tensor([1.0, 2.0, 3.0])
print(f1.dtype)

f2 = t1.to(torch.float32)

t2 = torch.tensor([
    [1, 2, 3],
    [4, 5, 6],
])

print(t2.shape)

print(t2.reshape(3, 2))

print(t2.reshape(3, 2)) # copying the data if necessary
print(t2.view(3, 2))    # the original data to be contiguous
print(t2.contiguous().view(3, 2))


print(t2.T)
print(t2.transpose(0, 1))

print(t2.matmul(t2.T))

print(t2 @ t2.T)
