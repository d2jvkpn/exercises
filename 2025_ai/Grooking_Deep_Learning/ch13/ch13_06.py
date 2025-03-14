#!/usr/bin/env python3

import numpy as np


class Tensor(object):
    def __init__(self, data, id=None, creators=[], creation_op=None, autograd=False):
        self.data = np.array(data)
        self.id = "r"+str(np.random.randint(0, 100000)) if id is None else id

        self.creators, self.creation_op = creators, creation_op
        self.autograd = autograd

        self.grad = None
        self.children = {}

        for c in creators:
            if self.id not in c.children:
                c.children[self.id] = 0
            c.children[self.id] += 1

    def set_id(self, id):
        old_id = self.id
        self.id = id

        for c in self.creators:
            c.children[self.id] = c.children.get(old_id, 0)
            del c.children[old_id]

    def backward(self, grad, grad_origin=None):
        #self.grad = grad

        #if self.creation_op == "add": # recursive
        #    self.creators[0].backward(grad)
        #    self.creators[1].backward(grad)

        if not self.autograd:
            return

        print(f"--> backward: id={self.id}, grad={grad}")
        if grad_origin is not None:
            if self.children[grad_origin.id] == 0:
                raise Exception("can't backprop more than once")
            else:
                self.children[grad_origin.id] -= 1

        if self.grad is None:
            self.grad = grad
        else:
            self.grad += grad

        if len(self.creators) == 0:
            return

        if self.all_children_grads() or grad_origin is None: # recursive
            print(f"--> backward: id={self.id}, grad={grad}")
            if self.creation_op == "add":
                self.creators[0].backward(self.grad, self)
                self.creators[1].backward(self.grad, self)
            elif self.creation_op == "neg":
                self.creators[0].backward(self.grad.__neg__())

    #def all_children_grads_accounted_for(self):
    def all_children_grads(self):
        for _, cnt in self.children.items():
            if cnt != 0: return False

        return True

    def __add__(self, other):
        print(f"--> operation: {self.id} + {other.id}")

        return Tensor(
          self.data + other.data,
          creators=[self, other], creation_op="add",
          autograd=(self.autograd or other.autograd),
        )


    def __neg__(self):
        if self.autograd:
            return Tensor(
              self.data * -1, id=f"(-{self.id})",
              creators=[self], creation_op="neg",
              autograd=True,
            )

        return Tensor(self.data * -1)

    def __repr__(self):
        # return str(self.data.__repr__())
        return f"{self.id}: {self.data.__str__()}"

    def __str__(self):
        return f"{self.id}: {self.data.__str__()}"

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
