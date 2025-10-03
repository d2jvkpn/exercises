#!/usr/bin/env python3

import numpy as np


# dN * wN -> p1
def neural_network(data, weights):
    pred = data.dot(weights) # np.dot(data, weights)
    return pred


weights = np.array([0.1, 0.2, 0.0])

toes = np.array([8.5, 9.5, 9.9, 9.0])   # number of toes,      weights[0]
wrate = np.array([0.65, 0.8, 0.8, 0.9]) # historical win rate, weights[1]
nfans = np.array([1.2, 1.3, 0.5, 1.0])  # number of fans,      weights[2]

print("~~~ inputs: [number_of_toes, historical_win_rate, number_of_fans]")
print("~~~ weights: {}".format(weights))
print("~~~ output: win_predication")

print()
for i in range(len(toes)):
    x = np.array([toes[i], wrate[i], nfans[i]])
    y = neural_network(x, weights)

    print(f"--> I{(i+1):02d}, inputs={x}, output={y:.3f}")

print()
print(np.array([toes, wrate, nfans]).T.dot(np.array(weights)))
