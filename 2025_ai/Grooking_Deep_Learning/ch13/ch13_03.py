#!/usr/bin/env python3

import numpy as np


class Tensor(object):
    def __init__(self, data, creators=None, creation_op=None):
       self.data = np.array(data)
       self.creators = creators
       self.creation_op = creation_op
       self.grad = None

    def __add__(self, other):
        return Tensor(self.data + other.data, creators=[self, other], creation_op="add")

    def __repr__(self):
        return str(self.data.__repr__())

    def __str__(self):
        return self.data.__str__()

    def backward(self, grad):
        self.grad = grad

        if self.creation_op == "add":
            # recursive
            self.creators[0].backward(grad)
            self.creators[1].backward(grad)


####
x = Tensor([1, 2, 3, 4, 5])
y = Tensor([2, 2, 2, 2, 2])

z = x + y
z.backward(Tensor(np.array([1, 1, 1, 1, 1])))

print(x.grad)
print(y.grad)
print(z.creators)

print(z.creation_op)

####
a = Tensor([1, 2, 3, 4, 5])
b = Tensor([2, 2, 2, 2, 2])
c = Tensor([5, 4, 3, 2, 1])
d = Tensor([-1, -2, -3, -4, -5])

e = a + b
f = c + d
g = e + f
g.backward(Tensor(np.array([1, 1, 1, 1, 1])))

print(a.grad)
print(e)
print(g)
