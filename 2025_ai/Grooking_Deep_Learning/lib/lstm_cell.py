#!/usr/bin/env python3

from .tensor import Tensor
from .layer import Layer, Linear

import numpy as np


class LSTMCell(Layer):
    def __init__(self, n_inputs, n_hidden, n_output):
        super().__init__()

        self.n_inputs = n_inputs
        self.n_hidden = n_hidden
        self.n_output = n_output

        self.xf = Linear(n_inputs, n_hidden)
        self.xi = Linear(n_inputs, n_hidden)
        self.xo = Linear(n_inputs, n_hidden)
        self.xc = Linear(n_inputs, n_hidden)

        self.hf = Linear(n_hidden, n_hidden, bias=False)
        self.hi = Linear(n_hidden, n_hidden, bias=False)
        self.ho = Linear(n_hidden, n_hidden, bias=False)
        self.hc = Linear(n_hidden, n_hidden, bias=False)

        self.w_ho = Linear(n_hidden, n_output, bias=False)

        self.parameters += self.xf.get_parameters()
        self.parameters += self.xi.get_parameters()
        self.parameters += self.xo.get_parameters()
        self.parameters += self.xc.get_parameters()

        self.parameters += self.hf.get_parameters()
        self.parameters += self.hi.get_parameters()
        self.parameters += self.ho.get_parameters()
        self.parameters += self.hc.get_parameters()

        self.parameters += self.w_ho.get_parameters()

    def forward(self, d, hidden):
        prev_hidden = hidden[0]
        prev_cell = hidden[1]

        f = (self.xf.forward(d) + self.hf.forward(prev_hidden)).sigmoid()
        i = (self.xi.forward(d) + self.hi.forward(prev_hidden)).sigmoid()
        o = (self.xo.forward(d) + self.ho.forward(prev_hidden)).sigmoid()
        g = (self.xc.forward(d) + self.hc.forward(prev_hidden)).tanh()
        c = f * prev_cell + i * g

        h = o * c.tanh()

        output = self.w_ho.forward(h)
        return output, (h, c)

    def init_hidden(self, batch_size=1):
        init_hidden = Tensor(np.zeros((batch_size, self.n_hidden)), autograd=True)
        init_cell = Tensor(np.zeros((batch_size, self.n_hidden)), autograd=True)
        init_hidden.data[:,0] += 1
        init_cell.data[:,0] += 1

        return (init_hidden, init_cell)
