#!/usr/bin/env python3

import sys
#sys.path.append('..')

from lib.tensor import Tensor
from lib.sgd import SGD
from lib.layer import Linear, Layer, Sequential, MSELoss

import numpy as np
np.random.seed(1)


data = Tensor([[0, 0], [0, 1], [1, 0], [1, 1]], autograd=True)
target = Tensor([[0], [1], [0], [1]], autograd=True)

model = Sequential([Linear(2, 3), Linear(3, 1)])
criterion = MSELoss()
optim = SGD(parameters=model.get_parameters(), alpha=0.1)


for n in range(20):
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
    print(f"==> I{n:04d}: loss={loss.data}")
