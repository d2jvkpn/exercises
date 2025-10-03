#!/usr/bin/env python3


alpha = 1.0
data = 0.85
goal = 1.0
weight = 0.1

for n in range(50):
    n += 1
    pred = data * weight
    delta = pred - goal
    error = delta ** 2

    if error < 1e-9:
        break

    weight_delta = delta * data     # 误差缩放
    weight -= weight_delta * alpha  # 负值反转

    print(f"--> pred={pred:.6f}, error={error:.6f}, delta={delta:.6f}, weight={weight:.6f}")
