#!/usr/bin/env python3

import sys, random, math
#sys.path.append('..')

from lib.tensor import Tensor
from lib.sgd import SGD
from lib.rnn_cell import RNNCell
from lib.layer import Embedding, CrossEntropyLoss

import numpy as np
np.random.seed(1)

#### 1. load data
with open("data/tasksv11/en/qa1_single-supporting-fact_train.txt", 'r') as f:
    raw_lines = f.readlines()

tokens, m, vocabs = list(), 0, set({"-"})

for line in raw_lines[:1000]:
    # skip the first word(numeric index)
    sent = line.lower().split()[1:]
    vocabs.update(set(sent))
    m = max(m, len(sent))
    tokens.append(sent)

for i in range(len(tokens)):
    sent = tokens[i]
    tokens[i] = ["-"] * (m - len(sent)) + sent #?? sent + ["-"] * (m - len(sent))

vocabs = list(vocabs)
vocabs.sort() # ??

print(f"==> Tokens: length={len(tokens)}, tokens[0:3]={tokens[0:3]}, m={m}")

word2index = {}
for i, w in enumerate(vocabs):
    word2index[w] = i

def words2indices(sent):
    return [word2index[w] for w in sent]

#### 2. setup parameters
indices = [words2indices(sent) for sent in tokens]
data = np.array(indices)

embed = Embedding(vocab_size=len(vocabs), dim=16)
model = RNNCell(n_inputs=16, n_hidden=16, n_output=len(vocabs))
criterion = CrossEntropyLoss()
parameters = model.get_parameters() + embed.get_parameters()
optim = SGD(parameters=parameters, alpha=0.01)

#### 3. trainning
for n in range(5000):
    n += 1
    batch_size, total_loss = 100, 0
    hidden = model.init_hidden(batch_size=batch_size)

    for t in range(5):
        x = Tensor(data[0:batch_size, t], autograd=True)
        y = embed.forward(x)
        output, hidden = model.forward(y, hidden=hidden)

    target = Tensor(data[0:batch_size, t+1], autograd=True)
    loss = criterion.forward(output, target)
    loss.backward(Tensor(np.ones_like(loss.data)))

    optim.step()
    total_loss += loss.data

    if n%200 == 0:
        p_correct = (target.data == np.argmax(output.data, axis=1)).mean()
        loss_frac = total_loss / (len(data) / batch_size)
        print(f"--> I{n:04d}: loss={loss_frac:.3f}, correct={p_correct}")
