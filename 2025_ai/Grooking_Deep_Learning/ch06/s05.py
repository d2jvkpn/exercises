#!/usr/bin/env python3

import numpy as np


alpha = 0.1

# 6x3
streelights = np.array([
  [1, 0, 1],
  [0, 1, 1],
  [0, 0, 1],
  [1, 1, 1],
  [0, 1, 1],
  [1, 0, 1],
])

# 1x6
walk_vs_stop = np.array([
  0,
  1,
  0,
  1,
  1,
  0,
])

weights = np.array([0.5, 0.48, -0.7])

data = streelights[0]
goal = walk_vs_stop[0]

for i in range(20):
    pred = data.dot(weights)
    delta = pred - goal
    error = delta ** 2
    weights -= (data * delta) * alpha

    print("--> I{}: error={}; pred={}; weights={}".format(
      i+1, np.round(error, 6), np.round(pred, 6), np.round(weights, 6),
    ))
