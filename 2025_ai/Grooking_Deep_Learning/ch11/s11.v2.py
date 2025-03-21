#!/usr/bin/env python3

import os, json
from os import path
import sys, random, math
from datetime import datetime
from collections import Counter
random.seed(1)

import numpy as np
import polars as pl
np.random.seed(1)


#### 1. data process
tokens = []
with open('reviews.txt') as f:
    # line.replace(".", " ")
    tokens = [line.split() for line in f.readlines()] # [["w11", "w12"], ["w21", "w22"]]

vocabs = set()
for s in tokens:
    vocabs.update(set(s))
vocabs = np.array(list(vocabs)) # np.array(["w1", "w2"])

word2index = {} # {"w1": 42}
for i in range(len(vocabs)):
    word2index[str(vocabs[i])] = i

dataset = list() # [[1, 2, 3], [4, 2, 1, 7]]
wordcnt = Counter()

for s in tokens:
    indices = list()
    for w in s:
        val = word2index.get(w)
        if val is None: continue
        indices.append(val)
        wordcnt[val] += 1

    dataset.append(indices)

concatenated = np.array([w for w in s for s in tokens ])

random.shuffle(dataset)

#### 2. init
alpha, iterations = 0.05, 2
hidden_size, window = 50, 2

weights_0_1 = (np.random.rand(len(vocabs), hidden_size) - 0.5) * 0.2  # range=(-0.1, 0.1)
weights_1_2 = np.zeros((len(vocabs), hidden_size))

target = np.zeros(window * 2 + 2)
target[0] = 1

def sigmoid(x):
    return 1 / (1 + np.exp(-x))

#### 3. trainning
def similar(target="beautiful"):
    index = word2index[target]

    scores = Counter()
    for w, i in word2index.items():
        difference = weights_0_1[i] - weights_0_1[index]
        squared = difference * difference
        scores[w] = -math.sqrt(sum(squared))

    return scores.most_common(10)

print(f"==> {datetime.now().astimezone().isoformat('T')} Trainning")
train_size = len(dataset) * iterations

for n in range(train_size):
    review = dataset[n%len(dataset)]
    n += 1

    for i in range(len(review)):
        # since it's really expensive to predict every vocabulary, we're only going to predict a random subset
        subset = list(np.random.choice(concatenated, window * 2 + 1))
        samples = [review[i]] + [word2index[w] for w in subset] # [w_idx1,... rand_idx1, rand_idx2, rand_idx3, rand_idx4, rand_idx5]
        # len(samples) = window * 2 + 2

        left = review[max(0, i - window) : i]
        right = review[i+1 : min(len(review), i + window)]

        # shape=(hidden_size,)
        layer_1 = np.mean(weights_0_1[left+right], axis=0)
        # print("~~~ layer_1", layer_1.shape)

        # (hidden_size,) * (window * 2 + 2, hidden_size).T = (window * 2 + 2,)
        layer_2 = sigmoid(np.dot(layer_1, weights_1_2[samples].T))
        #print("~~~ layer_2", layer_2.shape)

        delta_2 = layer_2 - target
        delta_1 = np.dot(delta_2, weights_1_2[samples])

        weights_0_1[left+right] -= delta_1 * alpha
        weights_1_2[samples] -= np.outer(delta_2, layer_1) * alpha

    if n % 250 == 0:
        progress = n/float(train_size)
        sys.stdout.write(f"--> {datetime.now().astimezone().isoformat('T')} progress={progress*100:.1f}%, predication={similar("terrible")}\n")

#### 4. predication
pred = similar("terrible")
print(f"==> {datetime.now().astimezone().isoformat('T')} Predication: word=terrible, similar={pred}")
# [('terrible', -0.0), ('lousy', -3.0263490922421665), ('horrible', -3.0825705973345094), ('bad', -3.4923930353861836), ('dreadful', -3.661124409465039), ('pathetic', -3.661640016721035), ('brilliant', -3.8498580668859512), ('poor', -3.9454628709482664), ('lame', -3.950512335499914), ('great', -4.085411395352522)]
# [('terrible', -0.0), ('horrible', -2.809275991332204), ('dreadful', -3.6002996139751993), ('lousy', -3.7001782014695337), ('horrid', -3.7722624314179667), ('brilliant', -4.085529438664867), ('horrendous', -4.086772942630519), ('stupid', -4.140400503357141), ('pitiful', -4.156934841206604), ('marvelous', -4.20513667055954)]

pred = similar("beautiful")
print(f"==> {datetime.now().astimezone().isoformat('T')} Predication: word=beautiful, similar={pred}")

#### 4. dump
os.makedirs("data", mode=511, exist_ok=True)

with open(path.join("data", "tokens.json"), 'w') as f:
    json.dump(tokens, f, indent=2)

with open(path.join("data", "word2index.json"), 'w') as f:
    json.dump(word2index, f, indent=2)

wts_0_1 = pl.from_numpy(weights_0_1)
wts_0_1.write_csv(path.join("data", "weights_0_1.tsv"), separator='\t')

wts_1_2 = pl.from_numpy(weights_1_2)
wts_1_2.write_csv(path.join("data", "weights_1_2.tsv"), separator='\t')
