#!/usr/bin/env python3

import os, json, argparse
from os import path
import sys, random, math
from datetime import datetime
from collections import Counter

import numpy as np
import polars as pl


parser = argparse.ArgumentParser(formatter_class=argparse.ArgumentDefaultsHelpFormatter)

parser.add_argument("--random_seed", type=int, default=1, help="random_seed")
parser.add_argument("--alpha", type=float, default=0.05, help="alpha")
parser.add_argument("--iterations", type=int, default=2, help="iterations")
parser.add_argument("--hidden_size", type=int, default=50, help="hidden_size")
parser.add_argument("--window", type=int, default=2, help="window")
parser.add_argument("--negative", type=int, default=5, help="negative")

args = parser.parse_args()

random.seed(args.random_seed)
np.random.seed(args.random_seed)


#### 1. data process
tokens, vocabs = [], set()
with open('reviews.txt') as f:
    # line.replace(".", " ")
    for line in f.readlines():
        sent = line.split()
        tokens.append(sent)
        vocabs.update(set(sent))

vocabs = list(vocabs) # fixed order
vocabs.sort()

word2index = {} # {"w1": 42}
for i in range(len(vocabs)):
    word2index[str(vocabs[i])] = i

dataset = list() # [[1, 2, 3], [4, 2, 1, 7]]

for s in tokens:
    indices = list()
    for w in s:
        val = word2index.get(w)
        if val is None: continue
        indices.append(val)

    dataset.append(indices)

concatenated = np.array([w for w in s for s in tokens])
random.shuffle(dataset)

#### 2. init
# shape=(vocabs, hidden_size)
weights_0_1 = (np.random.rand(len(vocabs), args.hidden_size) - 0.5) * 0.2 # range=(-0.1, 0.1)

# shape=(vocabs, hidden_size)
weights_1_2 = np.zeros((len(vocabs), args.hidden_size))

target = np.zeros(args.negative + 1)
target[0] = 1

def sigmoid(x):
    return 1 / (1 + np.exp(-x))

def similar(target="beautiful"):
    index = word2index[target]

    scores = Counter()
    for w, i in word2index.items():
        difference = weights_0_1[i] - weights_0_1[index]
        squared = difference * difference
        scores[w] = -math.sqrt(sum(squared))

    return scores.most_common(10)


#### 3. trainning
t = datetime.now().astimezone().isoformat('T')
print(f"==> {t} Trainning: args={args}")
train_size = len(dataset) * args.iterations

for n in range(train_size):
    review = dataset[n%len(dataset)]
    n += 1

    for i in range(len(review)):
        # since it's really expensive to predict every vocabulary, we're only going to predict a random subset
        subset = list(np.random.choice(concatenated, args.negative))
        # [w_idx1,... rand_idx1, rand_idx2, rand_idx3, rand_idx4, rand_idx5]
        samples = [review[i]] + [word2index[w] for w in subset]
        # len(samples) = negative + 1

        left = review[max(0, i - args.window) : i]              # length=window
        right = review[i+1 : min(len(review), i + args.window)] # length=window - 1

        # 1. forward propagation, [left_context..., right_context...] -> [1, 0, 0, 0...]
        # shape=(hidden_size,)
        layer_1 = np.mean(weights_0_1[left+right], axis=0)
        #print("~~~ step1", weights_0_1[left+right].shape, layer_1.shape)

        # (hidden_size,) * (negative + 1, hidden_size).T = (window * 2 + 2,)
        layer_2 = sigmoid(np.dot(layer_1, weights_1_2[samples].T)) # shape(hidden_size,)
        #print("~~~ step2", weights_1_2[samples].shape, layer_2.shape)

        # 2. backward propagation
        delta_2 = layer_2 - target                      # shape=(negative+1,)
        # (negative+1,) dot (negative+1, hidden_size)
        delta_1 = np.dot(delta_2, weights_1_2[samples]) # shape=(hidden_size,)

        # 3. update weights
        # (2*window - 1, hidden_size) -= (hidden_size)* alpha
        weights_0_1[left+right] -= delta_1 * args.alpha
        # (negative+1,) outer (hidden_size,) -> (negative+1, hidden_size)
        weights_1_2[samples] -= np.outer(delta_2, layer_1) * args.alpha

    if n % 250 == 0:
        progress = n / float(train_size)
        pred = similar("terrible")
        t = datetime.now().astimezone().isoformat('T')
        print(f"--> {t} progress={progress*100:.1f}%, predication={pred}")

#### 4. predication
predications = {}

pred = similar("terrible")
predications["terrible"] = pred
print(f"==> Predication: word=terrible, similar={pred}")
# [('terrible', -0.0), ('lousy', -3.0263490922421665), ('horrible', -3.0825705973345094), ('bad', -3.4923930353861836), ('dreadful', -3.661124409465039), ('pathetic', -3.661640016721035), ('brilliant', -3.8498580668859512), ('poor', -3.9454628709482664), ('lame', -3.950512335499914), ('great', -4.085411395352522)]
# [('terrible', -0.0), ('horrible', -2.809275991332204), ('dreadful', -3.6002996139751993), ('lousy', -3.7001782014695337), ('horrid', -3.7722624314179667), ('brilliant', -4.085529438664867), ('horrendous', -4.086772942630519), ('stupid', -4.140400503357141), ('pitiful', -4.156934841206604), ('marvelous', -4.20513667055954)]

pred = similar("beautiful")
predications["beautiful"] = pred
print(f"==> Predication: word=beautiful, similar={pred}")

#### 4. dump
os.makedirs("data", mode=511, exist_ok=True)

with open(path.join("data", "tokens.json"), 'w') as f:
    json.dump(tokens, f, indent=2)

with open(path.join("data", "word2index.json"), 'w') as f:
    json.dump(word2index, f, indent=2)

with open(path.join("data", "predications.json"), 'w') as f:
    json.dump(predications, f, indent=2)

wts_0_1 = pl.from_numpy(weights_0_1)
wts_0_1.write_csv(path.join("data", "weights_0_1.tsv"), separator='\t')

wts_1_2 = pl.from_numpy(weights_1_2)
wts_1_2.write_csv(path.join("data", "weights_1_2.tsv"), separator='\t')
