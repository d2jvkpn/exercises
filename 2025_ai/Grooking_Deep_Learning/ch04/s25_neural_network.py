#!/usr/bin/env python3


data = 2.0
goal = 0.8

alpha = 0.01 # 1.0, 0.1, 0.01, 0.001, 0.0001
weight = 0.5

total_steps = 500

for s in range(1, total_steps+1):
    pred = data * weight
    delta = pred - goal
    error = delta ** 2

    derivative = data * delta
    weight -= derivative * alpha

    print(f"--> {s:03d}: pred={pred:.6f}, error={error:.9f}, weight={weight:.6f}")

    if error < 1e-9:
        break
