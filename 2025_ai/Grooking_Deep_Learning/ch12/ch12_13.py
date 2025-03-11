#!/usr/bin/env python3

import sys
from collections import Counter

import numpy as np
np.random.seed(1)


### 1. load dataset
with open("data/tasksv11/en/qa1_single-supporting-fact_train.txt", 'r') as f:
    raw_lines = f.readlines()

tokens, vocabs = list(), set()

for line in raw_lines[:1000]:
    # skip the first word(numeric index)
    sent = line.lower().split()[1:]
    tokens.append(sent)
    vocabs.update(set(sent))

vocabs = list(vocabs)
vocabs.sort()

print(f"==> Tokens: length={len(tokens)}, tokens[0:3]={tokens[0:3]}")

word2index = {}
for i, w in enumerate(vocabs):
    word2index[w] = i

#### 2. prepare trainning
def words2indices(sent):
    return [word2index[w] for w in sent]

def softmax(a):
    b = np.exp(a - np.max(a))
    return b / b.sum(axis=0)

alpha = 0.001
iterations = 100
embed_size = 10

# word embeddings
embed = (np.random.rand(len(vocabs), embed_size) - 0.5) * 0.1 # range=(-0.05, 0.05), shape=(len(vocabs), embed_size)

# embedding -> embedding (initially the identity matrix)
recurrent = np.eye(embed_size) # shape=(embed_size, embed_size)

# sentence embedding for empty sentence
start = np.zeros(embed_size) # shape=(embed_size,)

# embedding -> output weights
decoder = (np.random.rand(embed_size, len(vocabs)) - 0.5) * 0.1 # range=(-0.05, 0.05), shape=(embed_size, len(vocabs))

# one hot lookups (for loss function)
one_hot = np.eye(len(vocabs)) # shape=(len(vocabs), len(vocabs))

def predict(sent):
    layers = [{"hidden": start}]
    loss = 0
    preds = list() # forward propagate

    for widx in sent:
        #print(layers)
        # try to predict the next term
        pred = softmax(layers[-1]['hidden'].dot(decoder)) # shape=(len(vocabs),)

        # generate the next hidden state
        hidden = layers[-1]['hidden'].dot(recurrent) + embed[widx] # shape=(embed_size,)

        layer = {"pred": pred, "hidden": hidden}

        # TODO: RuntimeWarning: invalid value encountered in add
        loss += -np.log(pred[widx]) # np.float64
        layers.append(layer)

    return layers, loss

#### 3. trainning
steps = len(tokens) * iterations

for n in range(steps):
    sent = words2indices(tokens[n%len(tokens)]) # list()
    size = float(len(sent))
    layers, loss = predict(sent)

    # back propagate
    for i in reversed(range(1, len(layers))):
        layer = layers[i]

        layer['output_delta'] = layer['pred'] - one_hot[sent[i-1]] # shape=(len(vocabs),)

        # if the last layer - don't pull from a later one becasue it doesn't exist
        layer['hidden_delta'] = layer['output_delta'].dot(decoder.T) # shape=(embed_size,)
        if i < len(layers)-1:
            layer['hidden_delta'] += layers[i+1]['hidden_delta'].dot(recurrent.T)

    layer = layers[0]
    layer['hidden_delta'] = layers[1]['hidden_delta'].dot(recurrent.T)


    # update weights
    start -= layers[0]['hidden_delta'] / size * alpha # shape=(embed_size,)

    for i in range(len(layers)-1):
        layer, layer_next = layers[i], layers[i+1]
        decoder -= np.outer(layer['hidden'], layer_next['output_delta']) / size * alpha
        embed[sent[i]] -= layer['hidden_delta'] * alpha / size
        recurrent -= np.outer(layer['hidden'], layer_next['hidden_delta']) / size * alpha

    n += 1
    if n % len(tokens) == 0:
        perplexity = np.exp(loss/size)
        print(f"--> I{n:06d}: perplexity={perplexity:.3f}")

#### 4. test
sent_idx = 4
layers, loss = predict(words2indices(tokens[sent_idx]))

print(f"==> Test: {tokens[sent_idx]}, layers={len(layers)}")
for i in range(len(layers) - 2):
    prev = tokens[sent_idx][i]
    target = tokens[sent_idx][i+1]
    layer = layers[i+1]
    pred = vocabs[layer["pred"].argmax()]
    print(f"--> input='{prev}', target='{target}', predication='{pred}'")
