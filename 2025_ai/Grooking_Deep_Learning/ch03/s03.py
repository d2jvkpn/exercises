#!/usr/bin/env python3


# dN * wN -> p1
def neural_network(d, weight):
    predication = d * weight
    return predication


number_of_toes = [8.5, 9.5, 10, 9]
d = number_of_toes[0]

weight = 0.1
pred = neural_network(d, weight)

print(f"{pred:.3f}")
