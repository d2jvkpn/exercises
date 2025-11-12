#!/usr/bin/env python3
import numpy as np


x = np.random.randn(5)
n = x.shape[0]
u = x.sum() / n        # μ, mean

variance = ((x - u) ** 2).sum() / n
sd = variance ** 0.5   # σ, standard deviation

y = (x - u) / sd

assert(np.std(y) - 1.0  < 1e-3)
print(np.std(y))
