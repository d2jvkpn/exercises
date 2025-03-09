#!/usr/bin/env python3

import os, random, math
from collections import Counter

import numpy as np

np.random.seed(1)


with open("data/tasksv11/en/qa1_single-supporting-fact_train.txt", 'r') as f:
    raw = f.readlines()

tokens = list()
vocabs = set()
train_size = 1000

for line in raw[0:train_size]:
    words = line.lower().split()[1:]
    tokens.append(words)
    vocabs.update(set(words))

word2index = {}
for i, w in enumerate(vocabs):
    word2index[w] = i

def words2indices(sentence):
    return [word2index[w] for w in sentence]

def softmax(x):
     e_x = np.exp(x - np.max(x))
     return e_x / e_x.sum(axis=0)

print(tokens[:3])

alpha = 0.001
embed_szie = 10
embed = (np.random.rand(len(vocabs), embed_szie) - 0.5) * 0.1 # word embeddings: range=[-0.05, 0.05], shape=(N, embed_szie)
recurrent = np.eye(embed_szie) # embedding -> embedding (initially the identity matrix): shape=(embed_szie, embed_szie)
start = np.eye(embed_szie)         # sentence embedding for empty sentence: shape=(embed_szie, embed_szie)

decoder = (np.random.rand(embed_szie, len(vocabs)) - 0.5) * 0.1 # embedding -> output weights: range=[-0.05, 0.05], shape=(embed_szie, N)
one_hot  = np.eye(len(vocabs))

def predict(sent):
    layers = [ {"hidden": start} ]
    loss = 0

   # forward propagate
    for i in range(len(sent)):
        hidden = layers[-1]["hidden"] # shape=(embed_szie, embed_szie)

        layer = {
            # state, try to predict the next term
            "pred": softmax(np.dot(hidden, decoder)), # shape=(embed_szie, N)
            # previous -> hidden, generate the next hidden state
            "hidden": np.dot(hidden, recurrent) + embed[sent[i]], # shape=(embed_szie, embed_szie) + shape=(embed_szie, )
        }

        loss += -np.log(layer["pred"][:, sent[i]])
        layers.append(layer)

    return layers, loss

# Red Sox defeat Yankees
for n in range(train_size * 3):
    sent = words2indices(tokens[n%len(tokens)][1:])
    layers, loss = predict(sent)

    for i in reversed(range(len(layers))):
        layer = layers[i]

        if i > 0:
            target = sent[i-1]
            layer["output_delta"] = layer["pred"] - one_hot[target]
            hidden_delta  = np.dot(layer["output_delta"], decoder.T)

            if i == len(layers)-1:
                layer["hidden_delta"] = hidden_delta
            else:
                layer["hidden_delta"] = hidden_delta + np.dot(layers[i+1]["hidden_delta"], recurrent.T)
        else:
             layer["hidden_delta"]  = np.dot(layers[i+1]["hidden_delta"], recurrent.T)


    start -= layers[0]["hidden_delta"] * alpha / float(len(sent))

    for i, layer in enumerate(layers[1:]):
        #print("~~~", layers[i]["hidden"].shape, layer["output_delta"].shape)
        decoder -= np.dot(layers[i]["hidden"], layer["output_delta"]) * alpha / float(len(sent))
        embed[sent[i]] -= layers[i]["hidden_delta"] * alpha / float(len(sent))
        recurrent -= np.dot(layers[i]["hidden"], layer["hidden_delta"]) * alpha / float(len(sent))

    if n%1000 == 0:
        print("Perplexity:", np.exp(loss/len(sent)))
