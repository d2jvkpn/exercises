#!/usr/bin/env python3

import sys, argparse
from collections import Counter

import numpy as np


parser = argparse.ArgumentParser(formatter_class=argparse.ArgumentDefaultsHelpFormatter)

parser.add_argument("--random_seed", type=int, help="random_seed", default=1)
parser.add_argument("--alpha", type=float, help="alpha", default=0.001)
parser.add_argument("--iterations", type=int, help="iterations", default=20)
parser.add_argument("--embed_size", type=int, help="embed_size", default=10)

args = parser.parse_args()
np.random.seed(args.random_seed)


### 1. load dataset
with open("data/tasksv11/en/qa1_single-supporting-fact_train.txt", 'r') as f:
    raw_lines = f.readlines()

tokens, vocabs = list(), set()

for line in raw_lines[:1000]:
    sent = line.lower().split()[1:] # skip the first word(numeric index)
    tokens.append(sent)
    vocabs.update(set(sent))

vocabs = list(vocabs)
#vocabs.sort()

print(f"==> 1. Tokens: length={len(tokens)}, tokens[0:3]={tokens[0:3]}, args={args}")

word2index = {}
for i, w in enumerate(vocabs):
    word2index[w] = i

def words2indices(sent):
    return [word2index[w] for w in sent]

#### 2. prepare trainning
def softmax(a):
    b = np.exp(a - np.max(a))
    return b / b.sum(axis=0)

# word embeddings: range=(-0.05, 0.05), shape=(len(vocabs), embed_size)
embed = np.random.rand(len(vocabs), args.embed_size) * 0.1 - 0.05

# embedding -> embedding (initially the identity matrix): shape=(embed_size, embed_size)
recurrent = np.eye(args.embed_size)

# sentence embedding for empty sentence: shape=(embed_size,)
start = np.zeros(args.embed_size)

# embedding -> output weights: range=(-0.05, 0.05), shape=(embed_size, len(vocabs))
decoder = np.random.rand(args.embed_size, len(vocabs)) * 0.1 - 0.05

# one hot lookups (for loss function): shape=(len(vocabs), len(vocabs))
one_hot = np.eye(len(vocabs))

def predict(sent):
    layers = [{"hidden": start}]
    loss = 0
    preds = list() # forward propagate

    for widx in sent:
        #print(layers)
        # try to predict the next term: shape=(len(vocabs),)
        pred = softmax(layers[-1]['hidden'].dot(decoder))
        if np.isnan(pred).any():
            break

        # generate the next hidden state: shape=(embed_size,)
        hidden = layers[-1]['hidden'].dot(recurrent) + embed[widx]

        layer = {"pred": pred, "hidden": hidden}

        # TODO: RuntimeWarning: invalid value encountered in add
        loss += -np.log(pred[widx] + np.finfo(float).eps) # np.float64
        layers.append(layer)

    return layers, loss

#### 3. trainning
steps = len(tokens) * args.iterations

for n in range(steps):
    n += 1

    # 1. forward propagation
    sent = words2indices(tokens[n%len(tokens)]) # list()
    size = float(len(sent))
    layers, loss = predict(sent)
    if len(sent) + 1 != len(layers):
        break

    if np.isnan(loss).any():
        break

    # 2. back propagation
    for i in reversed(range(1, len(layers))):
        layer = layers[i]
        layer['output_delta'] = layer['pred'] - one_hot[sent[i-1]] # shape=(len(vocabs),)

        # if the last layer - don't pull from a later one becasue it doesn't exist
        # shape=(embed_size,)
        layer['hidden_delta'] = layer['output_delta'].dot(decoder.T)

        if i < len(layers)-1:
            layer['hidden_delta'] += layers[i+1]['hidden_delta'].dot(recurrent.T)

    layer = layers[0]
    layer['hidden_delta'] = layers[1]['hidden_delta'].dot(recurrent.T)

    # 3. update weights: shape=(embed_size,)
    start -= layers[0]['hidden_delta'] * args.alpha / size

    for i, next_layer in enumerate(layers[1:]):
        layer = layers[i]
        decoder -= np.outer(layer['hidden'], next_layer['output_delta']) * args.alpha / size
        embed[sent[i]] -= layer['hidden_delta'] * args.alpha / size
        recurrent -= np.outer(layer['hidden'], next_layer['hidden_delta']) * args.alpha / size

    if n % len(tokens) == 0:
        perplexity = np.exp(loss/size)
        print(f"--> I{n:06d}: loss={loss:.3f}, perplexity={perplexity:.3f}")

print(f"{start}, {recurrent}") # embed, decoder

#### 4. test
sent_idx = 4
sent = tokens[sent_idx]
indices = words2indices(sent)
layers, loss = predict(words2indices(sent))

print(f"==> 3. Test: sentence={sent}, layers={len(layers)}, loss={loss:.3f}")

for i, layer in enumerate(layers[1:-1]):
    prev = tokens[sent_idx][i]
    pred = vocabs[layer["pred"].argmax()]
    top10 = np.argpartition(layer["pred"], -10)[-10:][::-1]
    preds = np.array(vocabs)[top10]
    print(f"--> input='{prev}', target='{sent[i+1]}', pred='{pred}'\n    preds={preds}")
