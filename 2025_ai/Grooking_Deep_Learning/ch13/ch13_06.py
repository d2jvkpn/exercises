#!/usr/bin/env python3

import numpy as np

from lib.tensor import Tensor


####
print("==> 1")
x = Tensor([1, 2, 3, 4, 5], "x")
y = Tensor([2, 2, 2, 2, 2], "y")

z = x + y; z.set_id("z")
print(f"x.children={x.children}, y.children={y.children}")

z.backward(Tensor(np.array([1, 1, 1, 1, 1])))

print(f"x.grad={x.grad}, y.grad={y.grad}, z.creators={z.creators}, z.creation_op={z.creation_op}")


####
print("==> 2")
a = Tensor([1, 2, 3, 4, 5], "a", autograd=True)
b = Tensor([2, 2, 2, 2, 2], "b", autograd=True)
c = Tensor([5, 4, 3, 2, 1], "c", autograd=True)
# d = Tensor([-1, -2, -3, -4, -5], "d")

d = a + b; d.set_id("d")
e = b + c; e.set_id("e")
f = d + e; f.set_id("f")

print(f"??? e.autograd={e.autograd}, e.creators={e.creators}, e.creation_op={e.creation_op}")

f.backward(Tensor([1, 1, 1, 1, 1]))
#f.backward(Tensor([1, 1, 1, 1, 1]))

print(f"b.grad: {b.grad}")

####
print("==> 3")
a = Tensor([1, 2, 3, 4, 5], "a", autograd=True)
b = Tensor([2, 2, 2, 2, 2], "b", autograd=True)
c = Tensor([5, 4, 3, 2, 1], "c", autograd=True)

d = a + (-b); d.set_id("d")
e = (-b) + c; e.set_id("e")

f = d + e; f.set_id("f")
f.backward(Tensor([1, 1, 1, 1]))

print(f"b.grad={b.grad}")
