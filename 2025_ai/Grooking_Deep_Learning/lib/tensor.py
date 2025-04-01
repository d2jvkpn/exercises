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

    def backward(self, grad=None, grad_origin=None):
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
            self._backward(grad)


    def _backward(self, grad):
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
        elif self.creation_op == "sigmoid":
            delta = Tensor(np.ones_like(self.grad.data)) - self
            return self.creators[0].backward(self * delta * self.grad)
        elif self.creation_op == "tanh":
            delta = Tensor(np.ones_like(self.grad.data)) - self * self
            return self.creators[0].backward(self * delta * self.grad)
        elif self.creation_op == "index_select":
            new_grad = np.zeros_like(self.creators[0].data)
            indices_ = self.index_select_indices.data.flatten()
            grad_ = grad.data.reshape(len(indices_), -1)
            for i in range(len(indices_)):
                new_grad[indices_[i]] += grad_[i]

            self.creators[0].backward(Tensor(new_grad))
        elif self.creation_op == "cross_entropy":
            dx = self.softmax_output - self.target_dist
            self.creators[0].backward(Tensor(dx))

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
        data = self.data + other.data

        if self.autograd and other.autograd:
            return Tensor(data, creators=[self, other], creation_op="add", autograd=True)
        return Tensor(data)

    def __neg__(self):
        data = self.data * -1

        if self.autograd:
            return Tensor(
              data, id=f"(-{self.id})", autograd=True, creators=[self], creation_op="neg",
            )
        return Tensor(data)

    def __sub__(self, other):
        data = self.data - other.data

        if self.autograd and other.autograd:
            return Tensor(data, autograd = True, creators=[self, other], creation_op="sub")
        return Tensor(data)

    def __mul__(self, other):
        data = self.data * other.data

        if self.autograd and other.autograd:
            return Tensor(data, autograd=True, creators=[self, other], creation_op="mul")

        return Tensor(data)

    def sum(self, dim):
        data = self.data.sum(dim)

        if self.autograd:
            return Tensor(data, autograd=True, creators=[self], creation_op=f"sum_{dim}")
        return Tensor(data)

    def expand(self, dim, copies):
        trans_cmd = list(range(0, self.data.ndim))
        trans_cmd.insert(dim, self.data.ndim)
        shape = list(self.data.shape) + [copies]
        data = self.data.repeat(copies).reshape(shape)
        data = data.transpose(trans_cmd)

        if self.autograd:
            return Tensor(data, creators=[self], creation_op=f"expand_{dim}", autograd=True)
        return Tensor(data)


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
        data = 1.0 / (1.0 + np.exp(-self.data))

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

    def cross_entropy(self, target_indices):
        temp = np.exp(self.data)
        softmax_output = temp / np.sum(temp, axis=self.data.ndim - 1, keepdims=True)
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
