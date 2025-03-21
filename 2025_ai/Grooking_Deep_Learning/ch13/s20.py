#!/usr/bin/env python3

from lib.tensor import Tensor
from lib.sgd import SGD
from lib.layer import Linear, Layer, Sequential, Embedding, Tanh, CrossEntropyLoss

import numpy as np
np.random.seed(1)


data = Tensor([1, 2, 1, 2], autograd=True)
target = Tensor([0, 1, 0, 1], autograd=True)

model = Sequential([Embedding(3, 3), Tanh(), Linear(3, 4)])
criterion = CrossEntropyLoss()
optim = SGD(parameters=model.get_parameters(), alpha=0.1)


for n in range(1000):
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

    print(f"==> I{n:05d}: loss={loss.data}")
