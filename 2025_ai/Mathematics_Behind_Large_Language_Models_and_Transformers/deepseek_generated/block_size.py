#!/usr/bin/env python3
import numpy as np

block_size = 4
tril = np.tril(np.ones((block_size, block_size), dtype=int))
print(tril)

self.register_buffer("tril", torch.tril(torch.ones(block_size, block_size)))
