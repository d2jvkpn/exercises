#!/usr/bin/env python3



def neural_network(data, weight):
    pred = data * weight
    return pred

alpha = 0.01
weight = 0.1

number_of_toes = [8.5]
win_or_lose_binary = [1] # (won!!!)

d = number_of_toes[0]
goal = win_or_lose_binary[0]

pred = neural_network(d, weight)
error = (pred - goal) ** 2
delta = pred - goal

weight_delta = d * delta
alpha = 0.01

weight -= weight_delta * alpha

error = error = (pred - goal) ** 2
print(f"==> Prediction: weight={weight:.3f}, error={error:.6f}")
