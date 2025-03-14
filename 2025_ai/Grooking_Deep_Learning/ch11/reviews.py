#!/usr/bin/env python3


from os import sys

import numpy as np
np.random.seed(1)


#### 1.
with open("reviews.txt", 'r') as f:
    reviews = [v.strip() for v in  f.readlines() if len(v.strip()) > 0]

with open("labels.txt", 'r') as f:
    labels = [v.strip() for v in  f.readlines() if len(v.strip()) > 0]
    labels = [1 if v == "positive" else 0 for v in labels]

assert(len(reviews) == len(labels))

#### 2.
vocabs = set()
for s in reviews:
    subset = set([w for w in s.split()]) # if len(w) > 1
    subset.discard(",")
    subset.discard(".")
    vocabs.update(subset)

vocabs = list(vocabs)
vocabs.sort()

word2index = {}
for i, w in enumerate(vocabs):
    word2index[w] = i

dataset = list()
for s in reviews:
    indices =  [word2index[w] for w in s.split() if w in word2index]
    dataset.append(list(set(indices))) # duplicates will be removed

print(f"--> reviews[0]:{reviews[0]}")
print(f"--> dataset[0]: {dataset[0]}")
print(f"--> labels[0]: {labels[0]}")

#### 3. prepare
def sigmoid(x):
    return 1/(1 + np.exp(-x))

alpha = 0.01
hidden_size = 100
iterations = 2

weights_0_1 = 0.2 * np.random.random((len(word2index), hidden_size)) - 0.1
weights_1_2 = 0.2 * np.random.random((hidden_size, 1)) - 0.1

train_size = int(len(dataset)*0.8)
test_size = len(dataset) - train_size

print(f"==> Parameters: alpha{alpha}, hidden_size={hidden_size}, iterations={iterations}, weights_0_1={weights_0_1.shape}, weights_1_2={weights_1_2.shape}")
print(f"==> Dataset: train_size={train_size}, test_size={test_size}")


#### 4. trainning
for n in range(iterations):
    n+=1
    correct = 0

    for i in range(train_size):
        layer_0, label = dataset[i], labels[i]
        layer_1 = sigmoid(np.sum(weights_0_1[layer_0], axis=0))
        layer_2 = sigmoid(np.dot(layer_1, weights_1_2))

        delta_2 = layer_2 - label
        delta_1 = np.dot(delta_2, weights_1_2.T)

        weights_0_1[layer_0] -= delta_1 * alpha
        weights_1_2 -= np.outer(layer_1, delta_2) * alpha
        correct += 1 if np.abs(delta_2) < 0.5 else 0

    train_acc = float(correct)/float(train_size)

    correct = 0
    for i in range(train_size, len(dataset)):
        layer_0, label = dataset[i], labels[i]
        layer_1 = sigmoid(np.sum(weights_0_1[layer_0], axis=0))
        layer_2 = sigmoid(np.dot(layer_1, weights_1_2))

        correct += 1 if (np.abs(layer_2 - label) < 0.5) else 0

    test_acc = correct/float(test_size)
    sys.stdout.write(f"--> I{n:03d}: train_accuracy={train_acc:.3f}, test_accuracy={test_acc:.3f}\n")

#### 5. similar
from collections import Counter
import math

def similar(target="beautiful"):
    index = word2index[target]
    scores = Counter()

    for w, i in word2index.items():
        difference = weights_0_1[i] - weights_0_1[index]
        squared = difference * difference
        scores[w] = -math.sqrt(squared.sum())

    return scores.most_common(10)

print(similar("beautiful"))
print(similar("atmosphere"))
print(similar("terrible"))
