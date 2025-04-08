#!/bin/user/env python3

import numpy as np


class Tensor (object):
    def __init__(self, data, autograd=False, creators=None, creation_op=None, id=None):
        self.data = np.array(data)
        self.autograd = autograd
        self.grad = None

        if id is None:
            self.id = np.random.randint(0, 1000000000)
        else:
            self.id = id

        self.creators = creators
        self.creation_op = creation_op
        self.children = {}

        if creators is not None:
            for c in creators:
                if self.id not in c.children:
                    c.children[self.id] = 1
                else:
                    c.children[self.id] += 1

    def all_children_grads_accounted_for(self):
        for _, cnt in self.children.items():
            if cnt != 0:
                return False
        return True

    def backward(self, grad=None, grad_origin=None):
        if not self.autograd:
            return

        if grad is None:
            grad = Tensor(np.ones_like(self.data))

        if grad_origin is not None:
            if self.children[grad_origin.id] == 0:
                return
                print(self.id)
                print(self.creation_op)
                print(len(self.creators))
                for c in self.creators:
                    print(c.creation_op)
                raise Exception("cannot backprop more than once")
            else:
                self.children[grad_origin.id] -= 1

        if self.grad is None:
            self.grad = grad
        else:
            self.grad += grad

        # grads must not have grads of their own
        assert(not grad.autograd)

        # only continue backpropping if there's something to
        # backprop into and if all gradients (from children)
        # are accounted for override waiting for children if
        # "backprop" was called on this variable directly
        if self.creators is not None and \
            (self.all_children_grads_accounted_for() or grad_origin is None):
            self._backward(grad)

    def _backward(self, grad):
        if self.creation_op == "add":
             self.creators[0].backward(self.grad, self)
             self.creators[1].backward(self.grad, self)

        if self.creation_op == "sub":
            self.creators[0].backward(Tensor(self.grad.data), self)
            self.creators[1].backward(Tensor(self.grad.__neg__().data), self)

        if self.creation_op == "mul":
            new = self.grad * self.creators[1]
            self.creators[0].backward(new, self)
            new = self.grad * self.creators[0]
            self.creators[1].backward(new, self)

        if self.creation_op == "mm":
            c0 = self.creators[0]
            c1 = self.creators[1]
            new = self.grad.mm(c1.transpose())
            c0.backward(new)
            new = self.grad.transpose().mm(c0).transpose()
            c1.backward(new)

        if self.creation_op == "transpose":
            self.creators[0].backward(self.grad.transpose())

        if "sum" in self.creation_op:
            dim = int(self.creation_op.split("_")[1])
            self.creators[0].backward(
                self.grad.expand(dim, self.creators[0].data.shape[dim]))

        if "expand" in self.creation_op:
            dim = int(self.creation_op.split("_")[1])
            self.creators[0].backward(self.grad.sum(dim))

        if self.creation_op == "neg":
            self.creators[0].backward(self.grad.__neg__())

        if self.creation_op == "sigmoid":
            ones = Tensor(np.ones_like(self.grad.data))
            self.creators[0].backward(self.grad * (self * (ones - self)))

        if self.creation_op == "tanh":
            ones = Tensor(np.ones_like(self.grad.data))
            self.creators[0].backward(self.grad * (ones - (self * self)))

        if self.creation_op == "cross_entropy":
            dx = self.softmax_output - self.target_dist
            self.creators[0].backward(Tensor(dx))

        if self.creation_op == "index_select":
            new_grad = np.zeros_like(self.creators[0].data)
            indices_ = self.index_select_indices.data.flatten()
            grad_ = grad.data.reshape(len(indices_), -1)

            for i in range(len(indices_)):
                new_grad[indices_[i]] += grad_[i]

            self.creators[0].backward(Tensor(new_grad))

    def __add__(self, other):
        data = self.data + other.data

        if self.autograd and other.autograd:
            return Tensor(data, autograd=True, creators=[self, other],  creation_op="add")
        return Tensor(data)

    def __neg__(self):
        data = self.data * -1

        if self.autograd:
            return Tensor(data, autograd=True, creators=[self], creation_op="neg")
        return Tensor(data)

    def __sub__(self, other):
        data = self.data - other.data

        if self.autograd and other.autograd:
            return Tensor(data, autograd=True, creators=[self, other], creation_op="sub")
        return Tensor(data)

    def __mul__(self, other):
        data = self.data * other.data

        if self.autograd and other.autograd:
            return Tensor(data, autograd=True, creators=[self, other], creation_op="mul")
        return Tensor(data)

    def sum(self, dim):
        data = self.data.sum(dim)

        if self.autograd:
            return Tensor(data, autograd=True, creators=[self], creation_op="sum_" + str(dim))
        return Tensor(data)

    def expand(self, dim, copies):
        trans_cmd = list(range(0, self.data.ndim))
        trans_cmd.insert(dim, self.data.ndim)

        new_data = self.data.repeat(copies).reshape(list(self.data.shape) + \
          [copies]).transpose(trans_cmd)

        if self.autograd:
            creation_op = "expand_"+str(dim)
            return Tensor(new_data, autograd=True, creators=[self], creation_op=creation_op)
        return Tensor(new_data)

    def transpose(self):
        data = self.data.transpose()

        if self.autograd:
            return Tensor(data, autograd=True, creators=[self], creation_op="transpose")
        return Tensor(data)

    def mm(self, x):
        data = self.data.dot(x.data)

        if self.autograd:
            return Tensor(data, autograd=True, creators=[self, x], creation_op="mm")
        return Tensor(data)

    def sigmoid(self):
        data = 1 / (1 + np.exp(-self.data))

        if self.autograd:
            return Tensor(data, autograd=True, creators=[self], creation_op="sigmoid")
        return Tensor(data)

    def tanh(self):
        data = np.tanh(self.data)

        if self.autograd:
            return Tensor(data, autograd=True, creators=[self], creation_op="tanh")
        return Tensor(data)

    def index_select(self, indices):
        data = self.data[indices.data]

        if self.autograd:
            new = Tensor(data, autograd=True, creators=[self], creation_op="index_select")
            new.index_select_indices = indices
            return new
        return Tensor(data)

    def softmax(self):
        data = np.exp(self.data)
        return data / np.sum(data, axis=self.data.ndim - 1, keepdims=True)

    def cross_entropy(self, target_indices):
        data = np.exp(self.data)
        softmax_output = data / np.sum(data, axis=self.data.ndim - 1, keepdims=True)

        t = target_indices.data.flatten()
        p = softmax_output.reshape(len(t), -1)
        target_dist = np.eye(p.shape[1])[t]
        loss = -(np.log(p) * target_dist).sum(1).mean()

        if self.autograd:
            out = Tensor(loss, autograd=True, creators=[self], creation_op="cross_entropy")
            out.softmax_output = softmax_output
            out.target_dist = target_dist
            return out

        return Tensor(loss)

    def __repr__(self):
        return str(self.data.__repr__())

    def __str__(self):
        return str(self.data.__str__())
