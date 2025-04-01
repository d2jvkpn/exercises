#!/usr/bin/env python3

import sys
#sys.path.append('..')

from tensor import Tensor, LSTMCell, SGD, Embedding, CrossEntropyLoss

import numpy as np
np.random.seed(1)


####
with open("shakespear.txt", 'r') as f:
    raw = f.read()

vocab = list(set(raw))
word2index = {}

for i, w in enumerate(vocab):
    word2index[w] = i
# len(word2index)=62

# shape=(99993,)
indices = np.array(list(map(lambda x: word2index[x], raw)))

####
embed = Embedding(vocab_size=len(vocab), dim=512)
model = LSTMCell(n_inputs=512, n_hidden=512, n_output=len(vocab))

criterion = CrossEntropyLoss()

optim = SGD(
  parameters=model.get_parameters() + embed.get_parameters(),
  alpha=0.05,
)

batch_size = 16
n_batches = int(indices.shape[0] / batch_size) # value=3124

bptt = 25 # Backpropagation Through Time, 16..=64
n_bptt = int((n_batches - 1) / bptt) # value=195

# shape=(n_batches, batch_size)=(3124,32)
batched_indices = indices[:batch_size*n_batches].reshape(batch_size, n_batches).T

# shape=(3123,32)
input_indices = batched_indices[0:-1]
target_indices = batched_indices[1:]

# shape=(n_bptt, bptt, batch_size)=(195, 16, 32)
input_batches = input_indices[:bptt*n_bptt].reshape(n_bptt, bptt, batch_size)
target_batches = target_indices[:bptt*n_bptt].reshape(n_bptt, bptt, batch_size)
min_loss = 1000

print(f""""
--> input_batches[0][0:5]:
    {input_batches[0][0:5]}

--> target_batches[0][0:5]:
    {target_batches[0][0:5]}
""")

def generate_sample(n=30, init_char=' '):
    s = ""
    hidden = model.init_hidden(batch_size=1)
    d = Tensor(np.array([word2index[init_char]]))

    for i in range(n):
        rnn_input = embed.forward(d)
        output, hidden = model.forward(rnn_input, hidden)
#         output.data *= 25
#         temp_dist = output.softmax()
#         temp_dist /= temp_dist.sum()

#         m = (temp_dist > np.random.rand()).argmax()
        m = output.data.argmax()
        c = vocab[m]
        input = Tensor(np.array([m]))
        s += c
    return s

def train(n):
    global min_loss

    batches_to_train = input_batches.shape[0]
    n_loss, total_loss = 0, 0.0
    hidden = model.init_hidden(batch_size=batch_size)

    for batch_i in range(batches_to_train):
        hidden = (Tensor(hidden[0].data, autograd=True), Tensor(hidden[1].data, autograd=True))
        losses = list()

        for t in range(bptt):
            d = Tensor(target_batches[batch_i][t], autograd=True)
            rnn_input = embed.forward(d)
            output, hidden = model.forward(rnn_input, hidden)

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

        if(batch_i % 1 == 0):
            sample = generate_sample(n=70, init_char='T').replace("\n"," ")
            log = f"--> I{(n+1):04d}: batch={batch_i}/{batches_to_train}, alpha={optim.alpha:.6f}"
            log += f", min_loss={min_loss:.6f}, epoch_loss={epoch_loss:.6f}"
            if(batch_i == 0):
               log += f"\n    sample=\"{sample}\""

            print(log)

    optim.alpha *= 0.99

for n in range(10):
    train(n)
