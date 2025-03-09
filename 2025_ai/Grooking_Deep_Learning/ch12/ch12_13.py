#!/usr/bin/env python3

import sys
from collections import Counter

import numpy as np
np.random.seed(1)


with open("data/tasksv11/en/qa1_single-supporting-fact_train.txt", 'r') as f:
    raw = f.readlines()

tokens, vocabs = list(), set()
for line in raw[:1000]:
    sent = line.lower().replace("\n", "").split()[1:]
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

embed = (np.random.rand(len(vocabs) ,embed_size) - 0.5) * 0.1 # word embeddings, range=(-0.05, 0.05)
recurrent = np.eye(embed_size) # embedding -> embedding (initially the identity matrix)
start = np.zeros(embed_size)      # sentence embedding for empty sentence
decoder = (np.random.rand(embed_size, len(vocabs)) - 0.5) * 0.1 # embedding -> output weights
one_hot = np.eye(len(vocabs)) # one hot lookups (for loss function)

def predict(sent):
    layers = [{"hidden": start}]
    loss = 0
    preds = list() # forward propagate

    for i in range(len(sent)):
        layer = {
            'pred': softmax(layers[-1]['hidden'].dot(decoder)), # try to predict the next term
            "hidden": layers[-1]['hidden'].dot(recurrent) + embed[sent[i]] # generate the next hidden state
        }

        #if np.isnan(layer['pred']).any(): break
        loss += -np.log(layer['pred'][sent[i]])
        layers.append(layer)

    return layers, loss

# forward
steps = len(tokens) * iterations
for n in range(steps):
    sent = words2indices(tokens[n%len(tokens)])
    size = float(len(sent))
    layers, loss = predict(sent)

    # back propagate
    for i in reversed(range(len(layers))):
        layer = layers[i]

        if(i > 0):
            target = sent[i-1]
            layer['output_delta'] = layer['pred'] - one_hot[target]
            new_hidden_delta = layer['output_delta'].dot(decoder.transpose())

            # if the last layer - don't pull from a later one becasue it doesn't exist
            if(i == len(layers)-1):
                layer['hidden_delta'] = new_hidden_delta
            else:
                layer['hidden_delta'] = new_hidden_delta + layers[i+1]['hidden_delta'].dot(recurrent.transpose())
        else:
            layer['hidden_delta'] = layers[i+1]['hidden_delta'].dot(recurrent.transpose())

    # update weights
    #if np.isnan(layers[0]['hidden_delta']).any(): break
    start -= layers[0]['hidden_delta'] * alpha / size

    for i, layer in enumerate(layers[1:]):
        prev = layers[i]
        decoder -= np.outer(prev['hidden'], layer['output_delta']) * alpha / size
        embed[sent[i]] -= prev['hidden_delta'] * alpha / size
        recurrent -= np.outer(prev['hidden'], layer['hidden_delta']) * alpha / size

    n+=1
    if n % 1000 == 0 or n == steps:
        perplexity = np.exp(loss/size)
        print(f"--> I{n:05d}: perplexity={perplexity:.3f}")
