#!/usr/bin/env python3
import time

import numpy as np
np.random.seed(42)


alpha = 0.1

# streelights: 6x3
dataset_x = np.array([
    [1, 0, 1],
    [0, 1, 1],
    [0, 0, 1],
    [1, 1, 1],
    [0, 1, 1],
    [1, 0, 1],
    #...
])

# walk_vs_stop: 1x6
dataset_y = np.array([
    0,
    1,
    0,
    1,
    1,
    0,
])

#weights = np.array([0.5, 0.48, -0.7])
weights = np.random.rand(3)

#data = dataset_x[0]
#goal = dataset_y[0]

def train(x, y):
    global weights

    pred = x.dot(weights)
    delta = pred - y

    error = delta ** 2
    derivative = x * delta
    weights -= derivative * alpha

    return pred, error


total_epoches = 100

print(f"==> Training")
best, error_min = None, np.inf
step = 0

for epoch in range(1, total_epoches+1):
    for i in range(len(dataset_x)):
        step += 1
        _, error = train(dataset_x[i], dataset_y[i])

        if error < error_min:
            error_min = error
            best = (epoch, step, weights.copy())

        print("--> {:03d}-{:03d}: error={:.9f}, weights={}".format(
            epoch,
            step,
            np.round(error, 9),
            [f"{v:.9f}" for v in weights],
        ))

        time.sleep(0.02)

pred = dataset_x.dot(best[2])
result = np.array([pred, dataset_y, pred - dataset_y]).T

print(f"\n==> Best model: error={error_min}, epoch={best[0]}, step={best[1]},")
print(f"    weights={weights}")

print(f"\n==> Predication(pred, goal, delta):\n{result}")
