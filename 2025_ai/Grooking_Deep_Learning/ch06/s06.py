#!/usr/bin/env python3

import numpy as np
np.random.seed(1)


# 6*3
streelights = np.array([
  [1, 0, 1],
  [0, 1, 1],
  [0, 0, 1],
  [1, 1, 1],
  [0, 1, 1],
  [1, 0, 1],
])

# 6
walk_vs_stop = np.array([
  0,
  1,
  0,
  1,
  1,
  0,
])

alpha = 0.1
weights = np.array([0.5, 0.48, -0.7])

print("==> streelights={}, walk_vs_stop={}, weights={}, alpha={}".format(
  streelights.shape, walk_vs_stop.shape, weights, alpha,
))

data = streelights[0]
goal = walk_vs_stop[0]

for i in range(40):
    for row in range(streelights.shape[0]):
        data = streelights[row]
        goal = walk_vs_stop[row]
        pred = data.dot(weights)

        delta = pred - goal
        error = delta ** 2
        weights -= data * delta * alpha

        if row == streelights.shape[0] - 1:
            print("--> I{}-{}: error={}, prediction={}, weights={};".format(
              i+1, row+1, np.round(error, 6), np.round(pred, 6), np.round(weights, 6),
            ))

print(f"==> delta={delta:.6f}")

# weights = [0.013892, 1.013815, -0.015993]

# input        goal    pressure
# [1, 0, 1]    0       - 0 -
# [0, 1, 1]    1       0 + +
# [0, 0, 1]    0       0 0 -
# [1, 1, 1]    1       + + +
# [0, 1, 1]    1       0 + +
# [1, 0, 1]    0       - 0 -
