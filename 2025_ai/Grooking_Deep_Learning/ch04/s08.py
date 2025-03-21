#!/usr/bin/env python3


data, goal = 0.5,  0.8
weight = 0.5

for n in range(50):
    n += 1
    pred = data * weight
    error = (pred - goal) ** 2
    adjust = (pred - goal) * data # direction and amount
    weight -= adjust

    print(f"--> I{n:02d}, error={error:.12f}, prediction={pred:.12f}")

error = error = (pred - goal) ** 2
print(f"==> weight={weight:.3f}, error={error:.12f}")
