#!/usr/bin/env python3

from .tensor import Tensor

import numpy as np


class SGD(object):
    def __init__(self, parameters, alpha=0.1):
        self.parameters = parameters
        self.alpha = alpha

    def zero(self):
        for p in self.parameters:
            p.grad.data *= 0

    def step(self, zero=True):
        for p in self.parameters:
            p.data -= p.grad.data * self.alpha

            if zero:
                p.grad.data *= 0

class Layer(object):
    def __init__(self):
        self.parameters = list()

    def get_parameters(self):
        return self.parameters

class Linear(Layer):
    def __init__(self, n_inputs, n_outputs, use_bias=True):
        super().__init__()
        self.use_bias = use_bias

        # standard normal distribution: mean(mu)=0, std(sigma)=1, np.std([1, 2, 3, 4])
        mu, sigma = 0, np.sqrt(2.0 / n_inputs)
        wts = mu + np.random.randn(n_inputs, n_outputs) * sigma
        self.weight = Tensor(wts, autograd=True)

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

    def forward(self, input):
        for layer in self.layers:
            input = layer.forward(input)
        return input

    def get_parameters(self):
        params = list()
        for l in self.layers:
            params += l.get_parameters()
        return params

class Embedding(Layer):
    def __init__(self, vocab_size, dim):
        super().__init__()

        self.vocab_size = vocab_size
        self.dim = dim

        # this random initialiation style is just a convention from word2vec
        self.weight = Tensor((np.random.rand(vocab_size, dim) - 0.5) / dim, autograd=True)

        self.parameters.append(self.weight)

    def forward(self, input):
        return self.weight.index_select(input)

class Tanh(Layer):
    def __init__(self):
        super().__init__()

    def forward(self, input):
        return input.tanh()

class Sigmoid(Layer):
    def __init__(self):
        super().__init__()

    def forward(self, input):
        return input.sigmoid()

class CrossEntropyLoss(object):
    def __init__(self):
        super().__init__()

    def forward(self, input, target):
        return input.cross_entropy(target)
