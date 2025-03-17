#!/usr/bin/env python3

from lib.tensor import Tensor
from lib.sgd import SGD

import numpy as np
np.random.seed(1)


data = Tensor([[0, 0], [0, 1], [1, 0], [1, 1]], autograd=True)

target = Tensor([[0], [1], [0], [1]], autograd=True)

wts = list()
wts.append(Tensor(np.random.rand(2, 3), autograd=True))
wts.append(Tensor(np.random.rand(3, 1), autograd=True))

optim = SGD(parameters=wts, alpha=0.1)

for i in range(50):
    pred = data.mm(wts[0]).mm(wts[1])
    loss = ((pred - target) * (pred - target)).sum(0)
    print(f"==> loss: {loss.data}")
    loss.backward(Tensor(np.ones_like(loss.data)))
    optim.step()

#print(f"==> loss={loss}")
