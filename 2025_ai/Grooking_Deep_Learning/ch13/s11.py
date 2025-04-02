#!/usr/bin/env python3

import sys
#sys.path.append('..')

from lib.tensor import Tensor
from lib.sgd import SGD

import numpy as np
np.random.seed(1)


data = Tensor([[0, 0], [0, 1], [1, 0], [1, 1]], autograd=True)
target = Tensor([[0], [1], [0], [1]], autograd=True)

wts = [
  # input layer -> hidden layer
  Tensor(np.random.rand(2, 3), autograd=True),
  # hidden layer -> predication layer
  Tensor(np.random.rand(3, 1), autograd=True),
]

optim = SGD(parameters=wts, alpha=0.1)

for n in range(20):
    n += 1

    pred = data.mm(wts[0]).mm(wts[1])
    if np.isinf(pred.data).any():
        break

    temp = ((pred - target) * (pred - target)).sum(0)
    if np.isinf(temp.data).any():
        break

    loss = temp
    print(f"==> I{n:04d}: loss={loss.data[0]:.6f}")
    # np.isnan(loss.data[0]):

    loss.backward(Tensor(np.ones_like(loss.data)))
    optim.step()

#print(f"==> loss={loss}")
