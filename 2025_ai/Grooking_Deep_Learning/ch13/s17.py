#!/usr/bin/env python3

import sys
#sys.path.append('..')

from lib.tensor import Tensor
from lib.sgd import SGD
from lib.layer import Linear, Layer, Sequential, Embedding, Tanh, Sigmoid, MSELoss

import numpy as np
np.random.seed(1)


data = Tensor([1, 2, 1, 2], autograd=True)
target = Tensor([[0], [1], [0], [1]], autograd=True)

model = Sequential([Embedding(5, 3), Tanh(), Linear(3, 1), Sigmoid()])
criterion = MSELoss()
optim = SGD(parameters=model.get_parameters(), alpha=0.5)


for n in range(500):
    n += 1

    pred = model.forward(data)
    if np.isinf(pred.data).any():
        break

    temp = criterion.forward(pred, target)
    if np.isinf(temp.data).any():
        break

    loss = temp
    loss.backward(Tensor(np.ones_like(loss.data)))
    optim.step()

    print(f"==> I{n:05d}: loss={loss.data[0]:.6f}")
