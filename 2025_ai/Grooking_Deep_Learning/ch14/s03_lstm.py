#!/usr/bin/env python3

import os, sys, signal, shelve, argparse
#sys.path.append("..")

from lib.chrono import Chrono
from lib.tensor import Tensor
from lib.layer import Layer, Embedding, CrossEntropyLoss, SGD
from lib.lstm_cell import LSTMCell

import numpy as np
np.random.seed(0)


#### 1. funcs
def shelve_dump(data: dict, filename: str):
    os.makedirs(os.path.dirname(filename), mode=511, exist_ok=True)

    with shelve.open(filename, 'c') as db:
        for k, v in data.items():
            db[k] = v

def shelve_load(filename: str) -> dict:
    with shelve.open(filename) as db:
        return { item[0]: item[1] for item in db.items() }

#### 2. load text data
# dataset from http://karpathy.github.io/2015/05/21/rnn-effectiveness/
with open('shakespear.txt','r') as f:
    raw_text = f.read()

vocab = list(set(raw_text))
vocab.sort()

word2index = {w: i for i, w in enumerate(vocab)}
indices = np.array(list(map(lambda x: word2index[x], raw_text)))

#### 3. init model parameters
n_iterations = 10
alpha = 0.05
apha_scale = 0.99
ndims = 512
batch_size = 16
bptt = 25
criterion = CrossEntropyLoss()

shelve_path = os.path.join("data", "shelve", 's03_lstm.shelve')
shelve_exists = os.path.isfile(shelve_path)

if not shelve_exists:
    embed = Embedding(vocab_size=len(vocab), dim=ndims)
    model = LSTMCell(n_inputs=ndims, n_hidden=ndims, n_output=len(vocab))
    model.w_ho.weight.data *= 0
    optim = SGD(parameters=model.get_parameters() + embed.get_parameters(), alpha=alpha)
    trainning_steps = []
    min_loss = 1000.0
else:
    print(f"==> {Chrono()} Loading from shelve file: path={shelve_path}")
    db = shelve_load(shelve_path)
    embed = db["embed"]
    model = db["model"]
    optim = db["optim"]
    min_loss = db["min_loss"]
    trainning_steps = db["trainning_steps"]
    end_at = db["end_at"] # db.get("end_at", "")
    print(f"--> last trainning: end_at={end_at}")

n_batches = int(indices.shape[0] / batch_size)
n_bptt = int((n_batches-1) / bptt)

batched_indices = indices[:n_batches*batch_size].reshape(batch_size, n_batches).transpose()

input_batches = batched_indices[0:-1][:n_bptt*bptt].reshape(n_bptt,bptt, batch_size)
target_batches = batched_indices[1:][:n_bptt*bptt].reshape(n_bptt, bptt, batch_size)

def dump(sig, frame):
    print(f"\n<== {Chrono()} Dumping to shelve file: path={shelve_path}")

    db = {
      "embed": embed,
      "model": model,
      "optim": optim,
      "min_loss": min_loss,
      "end_at": f"{Chrono()}",
      "trainning_steps": trainning_steps,
    }

    shelve_dump(db, shelve_path)
    sys.exit(0)

signal.signal(signal.SIGINT, dump)

#### 4. trainning
def generate_sample(n=30, init_char=' ', temperature=1.0):
    s = ""
    hidden = model.init_hidden(batch_size=1)
    d = Tensor(np.array([word2index[init_char]]))

    for _i in range(n):
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
    print(f"==> {Chrono()} Starting iteration {n}")

    for batch_i in range(batches_to_train):
        batch_n = batch_i + 1
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
        epoch_loss = np.exp(total_loss / batch_n)
        min_loss = min(epoch_loss, min_loss)

        if batch_n % 10 == 0 or batch_n == batches_to_train:
            now = Chrono()
            batch = f"{batch_n}/{batches_to_train}"
            trainning_steps.append([now, n, optim.alpha, batch, min_loss, epoch_loss])

            print(f"--> {now} I{n:03d}: alpha={optim.alpha:.3f}, batch={batch}", end="")
            print(f", min_loss={min_loss:.3f}, epoch_loss={epoch_loss:.3f}")

    optim.alpha *= apha_scale

for _i in range(n_iterations):
    n = _i + 1
    train(n)

    for temp in [0.5, 1.0, 1.5]:
        sample = generate_sample(n=70, init_char='T', temperature=temp)
        print(f"--> {Chrono()} sample: iteration={n}", end="")
        print(f", temperature={temp:.3f}, sample={repr(sample)}")

shelve_dump(db, shelve_path)
