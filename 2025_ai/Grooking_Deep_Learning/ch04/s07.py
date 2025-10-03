#!/usr/bin/env python3


data = 0.5
goal = 0.8
weight = 0.5
step_amount = 0.001

for n in range(1, 1001):
    pred = data * weight
    error = (pred - goal) ** 2

    if n % 100 == 0:
        print(f"--> I{n:04d}: prediction={pred:.3f}, error={error:.3f}")

    pred = data * (weight + step_amount)
    err_up = (pred - goal) ** 2

    pred = data * (weight - step_amount)
    err_down = (pred - goal) ** 2

    if error <= err_up and error <= err_down:
        break
    elif err_down < err_up:
        weight -= step_amount
    elif err_up < err_down:
        weight += step_amount

error = error = (pred - goal) ** 2
print(f"==> weight={round(weight, 6)}, error={error:.3f}")
