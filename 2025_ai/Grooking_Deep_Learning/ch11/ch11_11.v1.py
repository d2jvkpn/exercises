#!/usr/bin/env python3

import sys, random, math
from collections import Counter

import numpy as np

np.random.seed(1)
random.seed(1)

#### 1. data process
with open('reviews.txt') as f:
    reviews = f.readlines()

tokens = list(map(lambda x: x.replace(".", " ").split(), reviews))

wordcnt = Counter()
for s in tokens:
    for w in s:
        wordcnt[w] -= 1

vocabs = list(set(map(lambda x: x[0], wordcnt.most_common())))

word2index = {}
for i, w in enumerate(vocabs):
    word2index[w] = i

concatenated = list()
dataset = list()

for s in tokens:
    indices = list()
    for w in s:
        val = word2index.get(w)
        if val is None: continue
        indices.append(val)
        concatenated.append(val)

    dataset.append(indices)

concatenated = np.array(concatenated)
random.shuffle(dataset)

#### 2. init
alpha, iterations = 0.05, 2
hidden_size, window, negative = 50, 2, 5

weights_0_1 = (np.random.rand(len(vocabs), hidden_size) - 0.5) * 0.2
weights_1_2 = np.zeros((len(vocabs), hidden_size))

target = np.zeros(negative + 1)
target[0] = 1

def sigmoid(x):
    return 1 / (1 + np.exp(-x))

#### 3. trainning
def similar(target="beautiful"):
    index = word2index[target]

    scores = Counter()
    for w, i in word2index.items():
        difference = weights_0_1[i] - (weights_0_1[index])
        squared = difference * difference
        scores[w] = -math.sqrt(sum(squared))

    return scores.most_common(10)

for n, review in enumerate(dataset * iterations):
    n+1

    for i in range(len(review)):
        # since it's really expensive to predict every vocabulary, we're only going to predict a random subset
        idx = (np.random.rand(negative) * len(concatenated)).astype('int').tolist()
        samples = [review[i]] + list(concatenated[idx])

        left = review[max(0, i - window) : i]
        right = review[i+1 : min(len(review), i + window)]

        layer_1 = np.mean(weights_0_1[left+right], axis=0)
        layer_2 = sigmoid(np.dot(layer_1, weights_1_2[samples].T))
        delta_2 = layer_2 - target
        delta_1 = np.dot(delta_2, weights_1_2[samples])

        weights_0_1[left+right] -= delta_1 * alpha
        weights_1_2[samples] -= np.outer(delta_2, layer_1) * alpha

    if n % 250 == 0:
        size = len(dataset) * iterations
        progress = n/float(size)
        sys.stdout.write(f"--> progress={progress*100:.1f}%, predication={similar("terrible")}\n")

#### 4. predication
print(similar("terrible"))
# [('terrible', -0.0), ('lousy', -3.0263490922421665), ('horrible', -3.0825705973345094), ('bad', -3.4923930353861836), ('dreadful', -3.661124409465039), ('pathetic', -3.661640016721035), ('brilliant', -3.8498580668859512), ('poor', -3.9454628709482664), ('lame', -3.950512335499914), ('great', -4.085411395352522)]
# [('terrible', -0.0), ('horrible', -2.809275991332204), ('dreadful', -3.6002996139751993), ('lousy', -3.7001782014695337), ('horrid', -3.7722624314179667), ('brilliant', -4.085529438664867), ('horrendous', -4.086772942630519), ('stupid', -4.140400503357141), ('pitiful', -4.156934841206604), ('marvelous', -4.20513667055954)]
