#!/usr/bin/env python3

import numpy as np


indices = np.array(list(range(10_000))) # 10k

batch_size = 32
bptt = 16

n_batches = int(indices.shape[0]/batch_size)
n_bptt = int((n_batches - 1)/bptt) # 19

# shape=(32, 312), size=9984
batched_indices = indices[:n_batches*batch_size].reshape(batch_size, n_batches).T

# shape=(31, 312)
input_indices = batched_indices[0:-1]
target_indices = batched_indices[1:]


input_batches = input_indices[:n_bptt*bptt].reshape(n_bptt, bptt, batch_size)

target_batches = target_indices[:n_bptt*bptt].reshape(n_bptt, bptt, batch_size)
