#!/usr/bin/env python3


def neural_network(data, weights):
    pred = w_sum(data, weights)
    return pred

def w_sum(data, weights):
    assert(len(data) == len(weights))

    ans = 0

    for i in range(len(data)):
      ans += data[i] * weights[i]

    return ans


toes = [8.5, 9.5, 9.9, 9.0]   # number of toes,                   weights[0]

wlrec = [0.65, 0.8, 0.8, 0.9] # historical win rate (percentage), weights[1]
nfans = [1.2, 1.3, 0.5, 1.0]  # number of fans,                   weights[2]


weights = [0.1, 0.2, 0.0]


for i in range(len(toes)):
    d = [toes[i], wlrec[i], nfans[i]]
    pred = neural_network(d, weights)

    print("--> I{}, input_vector={}, predication={:.3f}".format(i, d, pred))
