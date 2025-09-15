#!/usr/bin/env python3
import math

import numpy as np

#### 1.
sent = [
  "The",
  "bank",
  "of",
  "the",
  "river",
  "was",
  "flooded",
]

tokens = [
  "[CLS]",
  "The",
  "bank",
  "of",
  "the",
  "river",
  "was",
  "flooded",
  "[SEP]",
]

d_model = 3 # embedding
d_head = 3

#### 2. Embedding vector: embedding + positional embedding
ev = np.array([    # (9, 3)
  [1.0, 0.2, 0.1], # [CLS]
  [0.2, 0.8, 0.1], # The
  [0.2, 0.1, 0.9], # bank
  [0.9, 1.0, 0.2], # of
  [0.2, 0.8, 1.0], # the
  [0.0, 0.9, 0.9], # river
  [0.3, 0.8, 0.9], # was
  [0.1, 0.9, 0.8], # flooded
  [0.0, 0.3, 0.1], # [SEP]
])

wts_query = np.array([ # (3, 3)
  [0.0, 0.0,   0.0],
  [0.0, 15.33, 0.0],
  [0.0, 0.0,   -13.11],
])

wts_key = np.array([
  [0.1, 0.2, 0.3],
  [0.4, 0.5, 0.6],
  [0.7, 0.8, 0.9],
])

wts_value = np.array([
  [0.7, 0.8, 0.9],
  [1.0, 1.1, 1.2],
  [1.3, 1.4, 1.5],
])


#### 3.
query_output = ev @ wts_query # shape=(9, 3)
assert len(tokens) == query_output.shape[0]

key_output = ev @ wts_key     # shape=(9, 3)
assert len(tokens) == key_output.shape[0]

value_output = ev @ wts_value # shape=(9, 3)
assert len(tokens) == value_output.shape[0]

#### 4.
attn_raw = query_output @ key_output.T * 1.0 / math.sqrt(d_head) # shape=(9, 9)

def softmax(x):
    exp_x = np.exp(x - np.max(x))
    return exp_x / np.sum(exp_x)

def attn_scores_of_word(word):
    word_attn = attn_raw[tokens.index(word)]

    print(f"--> Attention scores of word:")
    for i in range(len(word_attn)):
        print(f"{word}\t{tokens[i]}\t{word_attn[i]}")

attn_scores_of_word("river")
attn_scores_of_word("bank")
attn_scores_of_word("flooded")

attn_softmax = np.array([softmax(v) for v in attn_raw])
# dropout, ...

#### 5.
attn_output = attn_softmax @ value_output # shape=(9, 3)
print(result)
