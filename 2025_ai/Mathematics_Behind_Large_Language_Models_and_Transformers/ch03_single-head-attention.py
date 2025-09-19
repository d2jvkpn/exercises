#!/usr/bin/env python3
import math

import numpy as np

#### 1. tokens
sent = ["The", "bank", "of", "the", "river", "was", "flooded"]
tokens = ["[CLS]", "The", "bank", "of", "the", "river", "was", "flooded", "[SEP]"]

T = len(tokens) # sequence length
d_model = 3     # embedding, hidden layer
n_heads = 1     # number of heads
head_size = 3   # head size

assert d_model == n_heads * head_size

#### 2. Embedding vector: embedding + positional embedding: (T, d_model)
embedding = np.array([    # (9, 3)
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

# (d_model, head_size)
Wq = np.array([ # (3, 3)
  [0.0, 0.0,   0.0],
  [0.0, 15.33, 0.0],
  [0.0, 0.0,   -13.11],
])

Wk = np.array([
  [0.1, 0.2, 0.3],
  [0.4, 0.5, 0.6],
  [0.7, 0.8, 0.9],
])

Wv = np.array([
  [0.7, 0.8, 0.9],
  [1.0, 1.1, 1.2],
  [1.3, 1.4, 1.5],
])

#### 3. (T, d_model) @ (d_model, n_heads) -> (T, n_heads)
Q = embedding @ Wq # shape=(9, 3)
assert len(tokens) == Q.shape[0]

K = embedding @ Wk     # shape=(9, 3)
assert len(tokens) == K.shape[0]

V = embedding @ Wv # shape=(9, 3)
assert len(tokens) == V.shape[0]

#### 4. (T, n_heads) @ (n_heads, T) -> (T, T)
scale = 1.0 / math.sqrt(head_size)
scores = Q @ K.T * scale

causal_mask = np.triu(np.ones((T, T), dtype=bool), k=1)
scores = np.where(causal_mask, -np.inf, scores)
print(f"scores: {scores}")

def softmax(x):
    #exp_x = np.exp(x - np.max(x))
    #return exp_x / np.sum(exp_x)
    x_max = np.max(x, axis=-1, keepdims=True)
    e_x = np.exp(x - x_max)
    return e_x / np.sum(e_x, axis=-1, keepdims=True)

def scores_of_word(scores, word):
    word_scores = scores[tokens.index(word)]

    print(f"--> Attention scores of word:")
    for i in range(len(scores)):
        print(f"{word}\t{tokens[i]}\t{scores[i].round(3)}")

def dropout(x, drop_prob=0.5, training=True):
    if not training or drop_prob == 0.0:
        return x

    keep_prob = 1 - drop_prob
    mask = (np.random.rand(*x.shape) < keep_prob).astype(np.float32)
    return (x * mask) / keep_prob

scores_of_word(scores, "river")
scores_of_word(scores, "bank")
scores_of_word(scores, "flooded")

attn = softmax(scores)
print(f"attention = softmax(scores): \n{attn.round(3)}")
print(f"sum: {attn.sum(axis=-1).round(3)}")

attn = dropout(attn, drop_prob=0.2)
print(f"attention = dropout(attention): \n{attn.round(3)}")
print(f"sum: {attn.sum(axis=-1).round(3)}")

#### 5.
Ho = attn @ V # shape=(9, 3)
print(f"Head output: {Ho.shape}")
print(f"{Ho.round(3)}")
