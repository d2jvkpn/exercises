#!/usr/bin/env python3


alpha = 0.1 # 1.0, 0.1, 0.01, 0.001, 0.0001

weight = 0.5
data, goal = 2.0, 0.8

for n in range(20):
    n += 1
    pred = data * weight
    delta = pred - goal
    error = delta ** 2
    derivative = data * delta
    weight -= derivative * alpha

    print(f"--> pred={pred:.6f}, error={error:.6f}, weight={weight:.6f}")
