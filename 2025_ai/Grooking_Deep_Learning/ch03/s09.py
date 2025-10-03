#!/usr/bin/env python3

import numpy as np

def neural_network(data, weights):
    pred = vect_x_mat(data, weights)
    return pred

# dN * wN -> p1
def w_sum(a, b):
    assert(len(a) == len(b))
    output = 0

    for i in range(len(a)):
        output += a[i] * b[i]

    return output

# np.dot(dN, wNM) -> pM
# np.dot(dKN, wNM) -> pKM
def vect_x_mat(vect, matrix):
    assert(len(vect) == len(matrix))
    output = [0.0 for _ in range(len(matrix))]

    for i in range(len(matrix)):
        output[i] = w_sum(vect, matrix[i])

    # [hurt, win, sad]
    return [round(float(v), 3) for v in output]


# toes % win # fans, 3x3
weights = [
  [0.1, 0.1, -0.3], # toes,wrates,nfans -> hurt?
  [0.1, 0.2, 0.0],  # toes,wrates,nfans -> win?
  [0.0, 1.3, 0.1],  # toes,wrates,nfans -> sad?
]

# 3x4
toes = [8.5, 9.5, 9.9, 9.0]    # number of toes
wrates = [0.65, 0.8, 0.8, 0.9] # historical win rate
nfans = [1.2, 1.3, 0.5, 1.0]   # number of fans

print("~~~ inputs: [number_of_toes, win_rate, number_of_fans]")
print("~~~ weights: {}".format(weights))
print("~~~ inputs: [hurt, win, sad]")

print()
for i in range(len(toes)):
    d = np.array([toes[i], wrates[i], nfans[i]])
    pred = neural_network(d, weights)

    print(f"--> I{i:02d}: inputs={d}, outputs={pred}")

####
data = np.array([toes, wrates, nfans]).T
wts = np.array(weights).T

pred = np.dot(data, wts)
print(f"==> np.dot((S, 3), (3, P))=(S, P), pred={pred}")
