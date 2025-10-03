#!/usr/bin/env python3


alpha = 0.01 # 1.0, 0.1, 0.01, 0.001, 0.0001
weight = 0.5
data = 2.0
goal = 0.8

total_steps = 100

for n in range(1, total_steps+1):
    pred = data * weight
    delta = pred - goal
    error = delta ** 2

    if error < 1e-6:
        break

    derivative = data * delta
    weight -= derivative * alpha

    print(f"--> {n:03d}: pred={pred:.6f}, error={error:.6f}, weight={weight:.6f}")
