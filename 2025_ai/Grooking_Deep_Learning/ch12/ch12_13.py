#!/usr/bin/env python3

import sys
from collections import Counter

import numpy as np
np.random.seed(1)


with open("data/tasksv11/en/qa1_single-supporting-fact_train.txt", 'r') as f:
    raw = f.readlines()

tokens, vocabs = list(), set()

for line in raw[:1000]:
    sent = line.lower().split()[1:]
    tokens.append(sent)
    vocabs.update(set(sent))

vocabs = list(vocabs)
vocabs.sort()
print(f"==> Tokens: length={len(tokens)}, tokens[0:3]={tokens[0:3]}")

word2index = {}
for i, w in enumerate(vocabs):
    word2index[w] = i

def words2indices(sent):
    return [word2index[w] for w in sent]

def softmax(a):
    b = np.exp(a - np.max(a))
    return b / b.sum(axis=0)

alpha = 0.001
iterations = 100
embed_size = 10

embed = (np.random.rand(len(vocabs), embed_size) - 0.5) * 0.1 # word embeddings, range=(-0.05, 0.05)
recurrent = np.eye(embed_size) # embedding -> embedding (initially the identity matrix)
start = np.zeros(embed_size)   # sentence embedding for empty sentence
decoder = (np.random.rand(embed_size, len(vocabs)) - 0.5) * 0.1 # embedding -> output weights
one_hot = np.eye(len(vocabs)) # one hot lookups (for loss function)

def predict(sent):
    layers = [{"hidden": start}]
    loss = 0
    preds = list() # forward propagate

    for widx in sent:
        #print(layers)
        layer = {
            # try to predict the next term
            "pred": softmax(layers[-1]['hidden'].dot(decoder)), # shape=(embed_size, len(vocabs))
            # generate the next hidden state
            "hidden": layers[-1]['hidden'].dot(recurrent) + embed[widx], # shape=(embed_size,)
        }

        # TODO: RuntimeWarning: invalid value encountered in add
        loss += -np.log(layer['pred'][widx])
        layers.append(layer)

    return layers, loss

# forward
steps = len(tokens) * iterations

for n in range(steps):
    sent = words2indices(tokens[n%len(tokens)])
    size = float(len(sent))
    layers, loss = predict(sent)

    # back propagate
    for i in reversed(range(1, len(layers))):
        layer = layers[i]

        layer['output_delta'] = layer['pred'] - one_hot[sent[i-1]] # shape=(embed_size, len(vocabs))

        # if the last layer - don't pull from a later one becasue it doesn't exist
        layer['hidden_delta'] = layer['output_delta'].dot(decoder.T) # shape=(embed_size, embed_size)
        if i < len(layers)-1:
            layer['hidden_delta'] += layers[i+1]['hidden_delta'].dot(recurrent.T)

    layer = layers[0]
    layer['hidden_delta'] = layers[1]['hidden_delta'].dot(recurrent.T)


    # update weights
    start -= layers[0]['hidden_delta'] / size * alpha

    for i in range(len(layers)-1):
        layer, layer_next = layers[i], layers[i+1]
        decoder -= np.outer(layer['hidden'], layer_next['output_delta']) / size * alpha
        embed[sent[i]] -= layer['hidden_delta'] * alpha / size
        recurrent -= np.outer(layer['hidden'], layer_next['hidden_delta']) / size * alpha

    n+=1
    if n % len(tokens) == 0:
        perplexity = np.exp(loss/size)
        print(f"--> I{n:06d}: perplexity={perplexity:.3f}")

sent_index = 4
layers, loss = predict(words2indices(tokens[sent_index]))
print(tokens[sent_index])

for i, layer in enumerate(layers[1:-1]):
    input = tokens[sent_index][i]
    target = tokens[sent_index][i+1]
    pred = vocabs[layer["pred"].argmax()]
    print(f"Prev input={input}, target={target}, pred={pred}")
