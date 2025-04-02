#!/usr/bin/env python3

import os, sys, signal, shelve
#sys.path.append("..")

from lib.chrono import Chrono
from lib.tensor import Tensor
from lib.layer import Layer, Embedding, CrossEntropyLoss, SGD
from lib.models import LSTMCell

import numpy as np


#### 1.
np.random.seed(0)

def shelve_dump(data: dict, filename: str):
    os.makedirs(os.path.dirname(filename), mode=511, exist_ok=True)

    with shelve.open(filename, 'c') as db:
        for k, v in data.items():
            db[k] = v

def shelve_load(filename: str) -> dict:
    with shelve.open(filename) as db:
        return { item[0]: item[1] for item in db.items() }

#### 2. 
# dataset from http://karpathy.github.io/2015/05/21/rnn-effectiveness/
with open('shakespear.txt','r') as f:
    raw_text = f.read()

vocab = list(set(raw_text))
vocab.sort()

word2index = {w: i for i, w in enumerate(vocab)}
indices = np.array(list(map(lambda x: word2index[x], raw_text)))

#### 3. 
criterion = CrossEntropyLoss()
batch_size = 16
bptt = 25
n_batches = int(indices.shape[0] / batch_size)

batched_indices = indices[:n_batches*batch_size].reshape(batch_size, n_batches).transpose()

input_batched_indices = batched_indices[0:-1]
target_batched_indices = batched_indices[1:]

n_bptt = int((n_batches-1) / bptt)
input_batches = input_batched_indices[:n_bptt*bptt].reshape(n_bptt,bptt, batch_size)
target_batches = target_batched_indices[:n_bptt*bptt].reshape(n_bptt, bptt, batch_size)

shelve_path = os.path.join("data", "shelve", 's03.shelve')
shelve_exists = os.path.isfile(shelve_path)

if shelve_exists:
    print(f"==> {Chrono()} load shelve: path={shelve_path}")
    db = shelve_load(shelve_path)
    embed = db["embed"]
    model = db["model"]
    optim = db["optim"]
    min_loss = db["min_loss"]
else:
    embed = Embedding(vocab_size=len(vocab), dim=512)
    model = LSTMCell(n_inputs=512, n_hidden=512, n_output=len(vocab))
    model.w_ho.weight.data *= 0
    optim = SGD(parameters=model.get_parameters() + embed.get_parameters(), alpha=0.05)
    min_loss = 1000.0

def dump(sig, frame):
    print(f"\n<== {Chrono()} dumping: path={shelve_path}")

    db = {
      "embed": embed,
      "model": model,
      "optim": optim,
      "min_loss": min_loss,
    }

    shelve_dump(db, shelve_path)
    sys.exit(0)

signal.signal(signal.SIGINT, dump)

####
def generate_sample(n=30, init_char=' ', temperature=1.0):
    s = ""
    hidden = model.init_hidden(batch_size=1)
    d = Tensor(np.array([word2index[init_char]]))

    for i in range(n):
        rnn_input = embed.forward(d)
        output, hidden = model.forward(rnn_input, hidden=hidden)

        # Apply temperature scaling
        output.data /= temperature
        temp_dist = output.softmax()
        temp_dist /= temp_dist.sum()

        # Sample from the distribution
        m = np.random.choice(len(vocab), p=temp_dist[0])
        c = vocab[m]
        d = Tensor(np.array([m]))
        s += c

    return s

def train(n):
    global min_loss
    total_loss, n_loss = 0.0, 0.0

    hidden = model.init_hidden(batch_size=batch_size)
    # 分离上一轮的计算图
    hidden = (Tensor(hidden[0].data.copy()), Tensor(hidden[1].data.copy()))
    batches_to_train = len(input_batches)
    print(f"==> {Chrono()} starting iteration: {n}")

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
            print(f"--> {Chrono()} tranning: iteration={n}, alpha={optim.alpha:.3f}", end="")
            print(f", batch={batch_i+1:03d}/{batches_to_train}", end="")
            print(f", min_loss={min_loss:.3f}, epoch_loss={epoch_loss:.3f}")

    optim.alpha *= 0.99

for n in range(10):
    n += 1
    train(n)

    for temp in [0.5, 1.0, 1.5]:
        sample = generate_sample(n=70, init_char='T', temperature=temp)
        print(f"==> {Chrono()} sample: iteration={n}", end="")
        print(f", temperature={temp:.3f}, sample={repr(sample)}")
