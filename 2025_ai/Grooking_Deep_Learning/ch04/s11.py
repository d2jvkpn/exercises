#!/usr/bin/env python3


alpha = 1.0
data, goal = 0.85, 1.0
weight = 0.1

for n in range(10):
    n += 1
    pred = data * weight
    delta = pred - goal
    error = delta ** 2
    weight_delta = delta * data     # 误差缩放
    weight -= weight_delta * alpha  # 负值反转

    print(f"--> pred={pred:.6f}, error={error:.6f}, delta={delta:.6f}, weight={weight:.6f}")
