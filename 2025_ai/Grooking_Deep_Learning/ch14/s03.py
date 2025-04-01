#!/usr/bin/env python3

import sys

from lib.tensor import Tensor
from lib.layer import Layer, Embedding, CrossEntropyLoss, SGD
from lib.models import LSTMCell

import numpy as np

np.random.seed(0)

# dataset from http://karpathy.github.io/2015/05/21/rnn-effectiveness/
with open('shakespear.txt','r') as f:
    raw_text = f.read()

vocab = list(set(raw_text))
vocab.sort()

word2index = {w: i for i, w in enumerate(vocab)}
indices = np.array(list(map(lambda x: word2index[x], raw_text)))

embed = Embedding(vocab_size=len(vocab), dim=512)
model = LSTMCell(n_inputs=512, n_hidden=512, n_output=len(vocab))
model.w_ho.weight.data *= 0

criterion = CrossEntropyLoss()
optim = SGD(parameters=model.get_parameters() + embed.get_parameters(), alpha=0.05)

def generate_sample(n=30, init_char=' '):
    s = ""
    hidden = model.init_hidden(batch_size=1)
    d = Tensor(np.array([word2index[init_char]]))

    for i in range(n):
        rnn_input = embed.forward(d)
        output, hidden = model.forward(rnn_input, hidden=hidden)
#         output.data *= 25
#         temp_dist = output.softmax()
#         temp_dist /= temp_dist.sum()

#         m = (temp_dist > np.random.rand()).argmax()
        m = output.data.argmax()
        c = vocab[m]
        d = Tensor(np.array([m]))
        s += c

    return s

batch_size = 16
bptt = 25
n_batches = int((indices.shape[0] / (batch_size)))

batched_indices = indices[:n_batches*batch_size].reshape(batch_size, n_batches).transpose()

input_batched_indices = batched_indices[0:-1]
target_batched_indices = batched_indices[1:]

n_bptt = int((n_batches-1) / bptt)
input_batches = input_batched_indices[:n_bptt*bptt].reshape(n_bptt,bptt, batch_size)
target_batches = target_batched_indices[:n_bptt*bptt].reshape(n_bptt, bptt, batch_size)

min_loss = 1000.0
def train(n):
    global min_loss
    total_loss, n_loss = 0.0, 0.0

    hidden = model.init_hidden(batch_size=batch_size)
    batches_to_train = len(input_batches)

    for batch_i in range(batches_to_train):
        hidden = (Tensor(hidden[0].data, autograd=True), Tensor(hidden[1].data, autograd=True))
        losses = list()

        for t in range(bptt):
            d = Tensor(input_batches[batch_i][t], autograd=True)
            rnn_input = embed.forward(input=d)
            output, hidden = model.forward(input=rnn_input, hidden=hidden)

            target = Tensor(target_batches[batch_i][t], autograd=True)
            batch_loss = criterion.forward(output, target)

            if t == 0:
                losses.append(batch_loss)
            else:
                losses.append(batch_loss + losses[-1])

        loss = losses[-1]
        loss.backward()
        optim.step()
        total_loss += loss.data / bptt

        epoch_loss = np.exp(total_loss / (batch_i+1))
        min_loss = min(epoch_loss, min_loss)

        if (batch_i+1) % 10 == 0 or batch_i == batches_to_train-1:
            sample = generate_sample(n=70, init_char='T').replace("\n"," ")
            log = f"--> I{n:04d}: alpha={optim.alpha:.3f}, batch={batch_i+1}/{batches_to_train}"
            log += f", min_loss={min_loss:.3f}, epoch_loss={epoch_loss:.3f}"
            log += f'\n      sample="{sample}"'
            print(log)

    optim.alpha *= 0.99

for n in range(10):
    train(n+1)
