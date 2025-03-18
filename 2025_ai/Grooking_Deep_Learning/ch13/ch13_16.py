#!/usr/bin/env python3

from lib.tensor import Tensor
from lib.sgd import SGD
from lib.linear import Linear, Layer

import numpy as np
np.random.seed(1)


class Sequential(Layer):
    def __init__(self, layers=list()):
        super().__init__()
        self.layers = layers

    def add(self, layer):
        self.layers.append(layer)

    def forward(self, data):
        for layer in self.layers:
            data = layer.forward(data)

        return data

    def get_parameters(self):
        parameters = list()
        for layer in self.layers:
            parameters += layer.get_parameters()

        return parameters


class MSELoss(Layer):
    def __init__(self):
        super().__init__()

    def forward(self, pred, target):
        delta = pred - target
        return (delta * delta).sum(0)

class Tanh(Layer):
    def __init__(self):
        super().__init__()

    def forward(self, data):
        return data.tanh()

class Sigmoid(Layer):
    def __init__(self):
        super().__init__()

    def forward(self, data):
        return data.sigmoid()


data = Tensor([[0, 0], [0, 1], [1, 0], [1, 1]], autograd=True)
target = Tensor([[0], [1], [0], [1]], autograd=True)

model = Sequential([Linear(2, 3), Tanh(), Linear(3, 1), Sigmoid()])
criterion = MSELoss()
optim = SGD(parameters=model.get_parameters(), alpha=1)


for n in range(1000):
    n+=1

    pred = model.forward(data)
    if np.isinf(pred.data).any():
        break

    temp = criterion.forward(pred, target)
    if np.isinf(temp.data).any():
        break

    loss = temp
    loss.backward(Tensor(np.ones_like(loss.data)))
    optim.step()

    if n % 10 == 0:
        print(f"==> I{n:04d}: loss={loss.data}")
