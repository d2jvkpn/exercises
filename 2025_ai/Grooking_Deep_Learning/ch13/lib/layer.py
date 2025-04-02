from .tensor import Tensor

import numpy as np


class Layer(object):
    def __init__(self):
        self.parameters = list()

    def get_parameters(self):
        return self.parameters

class Linear(Layer):
    def __init__(self, n_inputs, n_outputs, bias=True):
        super().__init__()
        self.use_bias = bias

        W = np.random.randn(n_inputs, n_outputs) * np.sqrt(2.0 / n_inputs)

        self.weight = Tensor(W, autograd=True)
        if self.use_bias:
           self.bias = Tensor(np.zeros(n_outputs), autograd=True)

        self.parameters.append(self.weight)

        if self.use_bias:
           self.parameters.append(self.bias)

    def forward(self, input):
        if self.use_bias:
           return input.mm(self.weight) + self.bias.expand(0, len(input.data))
        return input.mm(self.weight)

class Sequential(Layer):
    def __init__(self, layers=list()):
        super().__init__()
        self.layers = layers

    def add(self, layer):
        self.layers.append(layer)

    def forward(self, data):
        for layer in self.layers:
            data = layer.forward(data)

        return data

    def get_parameters(self):
        parameters = list()
        for layer in self.layers:
            parameters += layer.get_parameters()

        return parameters

class MSELoss(Layer):
    def __init__(self):
        super().__init__()

    def forward(self, pred, target):
        delta = pred - target
        return (delta * delta).sum(0)

class Tanh(Layer):
    def __init__(self):
        super().__init__()

    def forward(self, data):
        return data.tanh()

class Sigmoid(Layer):
    def __init__(self):
        super().__init__()

    def forward(self, data):
        return data.sigmoid()

class Embedding(Layer):
    def __init__(self, vocab_size, dim):
        super().__init__()
        self.vocab_size = vocab_size
        self.dim = dim
        weight = (np.random.rand(vocab_size, dim) - 0.5) / dim
        self.weight = Tensor(weight, autograd=True)

    def forward(self, data):
        return self.weight.index_select(data)

class CrossEntropyLoss(Layer):
    def __init__(self):
        super().__init__()

    def forward(self, data, target):
        return data.cross_entropy(target)
