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

        #print(f"--> backward: id={self.id}, grad={grad}")
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
            #print(f"--> backward: id={self.id}, grad={grad}")
            self._backward()


    def _backward(self):
        if self.creation_op == "add":
            self.creators[0].backward(self.grad, self)
            self.creators[1].backward(self.grad, self)
        elif self.creation_op == "neg":
            self.creators[0].backward(self.grad.__neg__())
        elif self.creation_op == "sub":
            new = Tensor(self.grad.data)
            self.creators[0].backward(new, self)
            new = Tensor(self.grad.__neg__().data)
            self.creators[1].backward(new, self)
        elif self.creation_op == "mul":
            new = self.grad * self.creators[1]
            self.creators[0].backward(new, self)
            new = self.grad * self.creators[0]
            self.creators[1].backward(new, self)
        elif self.creation_op == "mm":
            act = self.creators[0]
            weights = self.creators[1]
            new = self.grad.mm(weights.transpose())
            act.backward(new)
            new = self.grad.transpose().mm(act).transpose()
            weights.backward(new)
        elif self.creation_op == "transpose":
            self.creators[0].backward(self.grad.transpose())
        elif "sum" in self.creation_op:
            dim = int(self.creation_op.split("_")[1])
            ds = self.creators[0].data.shape[dim]
            self.creators[0].backward(self.grad.expand(dim, ds))
        elif "expand" in self.creation_op:
            dim = int(self.creation_op.split("_")[1])
            self.creators[0].backward(self.grad.sum(dim))


    #def all_children_grads_accounted_for(self):
    def all_children_grads(self):
        for _, cnt in self.children.items():
            if cnt != 0: return False

        return True


    def __repr__(self):
        # return str(self.data.__repr__())
        return f"{self.id}: {self.data.__str__()}"

    def __str__(self):
        return f"{self.id}: {self.data.__str__()}"

    def __add__(self, other):
        #print(f"--> operation: {self.id} + {other.id}")

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

    def __sub__(self, other):
        if self.autograd and other.autograd:
            return Tensor(
              self.data - other.data, autograd = True,
              creators=[self, other], creation_op = "sub",
            )

        return Tensor(self.data - other.data)

    def __mul__(self, other):
        if self.autograd and other.autograd:
            return Tensor(
              self.data * other.data, autograd=True,
              creators=[self, other], creation_op="mul",
            )

        return Tensor(self.data * other.data)

    def sum(self, dim):
        if self.autograd:
            return Tensor(
              self.data.sum(dim), autograd=True,
              creators=[self], creation_op=f"sum_{dim}",
            )

        return Tensor(self.data.sum(dim))

    def expand(self, dim, copies):
        trans_cmd = list(range(0, self.data.ndim))
        trans_cmd.insert(dim, self.data.ndim)
        new_shape = list(self.data.shape) + [copies]
        new_data = self.data.repeat(copies).reshape(new_shape)
        new_data = new_data.transpose(trans_cmd)

        if self.autograd:
            return Tensor(new_data, creators=[self], creation_op=f"expand_{dim}", autograd=True)

        return Tensor(new_data)


    def transpose(self):
        if self.autograd:
            return Tensor(
              self.data.transpose(), autograd=True,
              creators=[self], creation_op="transpose",
            )

        return Tensor(self.data.transpose())

    def mm(self, x):
        if self.autograd:
            return Tensor(
              self.data.dot(x.data), autograd=True,
              creators=[self, x], creation_op="mm",
            )

        return Tensor(self.data.dot(x.data))
