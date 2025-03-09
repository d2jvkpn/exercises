#!/usr/bin/env python3

import sys
from collections import Counter

import numpy as np

np.random.seed(1)

with open("data/tasksv11/en/qa1_single-supporting-fact_train.txt", 'r') as f:
    raw = f.readlines()

tokens = list()
vocabs = set()

for line in raw[:1000]:
    sent = line.lower().replace("\n", "").split()[1:]
    tokens.append(sent)
    vocabs.update(set(sent))

vocabs = list(vocabs)
print(f"==> Tokens: length={len(tokens)}, tokens[0:3]={tokens[0:3]}")

word2index = {}
for i, w in enumerate(vocabs):
    word2index[w]=i
    
def words2indices(sentence):
    idx = list()
    for word in sentence:
        idx.append(word2index[word])
    return idx

def softmax(x):
    e_x = np.exp(x - np.max(x))
    return e_x / e_x.sum(axis=0)

alpha = 0.001
embed_size = 10
# word embeddings
embed = (np.random.rand(len(vocabs) ,embed_size) - 0.5) * 0.1

# embedding -> embedding (initially the identity matrix)
recurrent = np.eye(embed_size)

# sentence embedding for empty sentence
start = np.zeros(embed_size)

# embedding -> output weights
decoder = (np.random.rand(embed_size, len(vocabs)) - 0.5) * 0.1

# one hot lookups (for loss function)
one_hot = np.eye(len(vocabs))

def predict(sent):
    layers = [{"hidden": start}]
    loss = 0

    # forward propagate
    preds = list()
    for i in range(len(sent)):
        layer = {
            'pred': softmax(layers[-1]['hidden'].dot(decoder)), # try to predict the next term
            "hidden": layers[-1]['hidden'].dot(recurrent) + embed[sent[i]] # generate the next hidden state
        }

        loss += -np.log(layer['pred'][sent[i]])
        layers.append(layer)

    return layers, loss

# forward
iterations = 50000
for n in range(iterations):
    sent = words2indices(tokens[n%len(tokens)][1:])
    size = float(len(sent))
    layers, loss = predict(sent) 

    # back propagate
    for i in reversed(range(len(layers))):
        layer = layers[i]
        target = sent[i-1]

        if(i > 0):
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
    start -= layers[0]['hidden_delta'] * alpha / size

    for i, layer in enumerate(layers[1:]):
        prev = layers[i]
        decoder -= np.outer(prev['hidden'], layer['output_delta']) * alpha / size
        embed[sent[i]] -= prev['hidden_delta'] * alpha / size
        recurrent -= np.outer(prev['hidden'], layer['hidden_delta']) * alpha / size

    n+=1
    if n % 1000 == 0 or n == iterations:
        perplexity = np.exp(loss/size)
        print(f"--> I{n:05d}: perplexity={perplexity:.3f}")
