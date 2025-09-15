#!/usr/bin/env python3
import math

import numpy as np


# matrix
dims, size = 4, 100

#Positiona Encoding

#even_indices(pos=0, 2, 4..., dim=i)
#pe = math.sin(pos / size ** (2*  dim/dims))

#odd_indices(pos=1, 3, 5..., dim=i)
#pe = math.cos(pos / size ** (2*dim/dims))


def calc_pe(pos):
    def cal(d):
        v = d // 2 * 2
        v = pos / size**(2 * v / dims)
        return math.sin(v) if d % 2 == 0 else math.cos(v)

    return [round(cal(d), 2) for d in range(dims)]


sent = ["I", "am", "a", "robot"]

embeddings = np.array([
    [1.0, 0.5, 0.3, 0.0], # I
    [0.9, 0.8, 0.2, 1.0], # am
    [0.7, 0.1, 0.4, 0.0], # a
    [1.1, 0.4, 0.3, 1.0], # robot
])

pe = np.array([calc_pe(pos) for pos in range(len(sent))])

print(f"Positional Embedding: {pe}")

output = embeddings + pe

print(f"Output: {output}")
