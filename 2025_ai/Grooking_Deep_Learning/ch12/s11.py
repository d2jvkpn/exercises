#!/usr/bin/env python3

import numpy as np
np.random.seed(1)


def softmax(x_):
    x = np.atleast_2d(x_)
    val = np.exp(x)
    return val / np.sum(val, axis=1, keepdims=True)

word_vects = {
  "yankees": np.zeros((1, 3)),
  "bears": np.zeros((1, 3)),
  "braves": np.zeros((1, 3)),
  "red": np.zeros((1, 3)),
  "sox": np.zeros((1, 3)),
  "lose": np.zeros((1, 3)),
  "defeat": np.zeros((1, 3)),
  "bit": np.zeros((1, 3)),
  "tie": np.zeros((1, 3)),
}

sent2output = np.random.rand(3, len(word_vects))
identity = np.eye(3)

layer_0 = word_vects["red"]
layer_1 = layer_0.dot(identity) + word_vects["sox"]
layer_2 = layer_1.dot(identity) + word_vects["defeat"]

pred = softmax(layer_2.dot(sent2output))
print("==> pred:", pred)

y = np.array([1, 0, 0, 0, 0, 0, 0, 0, 0])

pred_delta = pred - y

delta_2 = pred_delta.dot(sent2output.T)
defeat_delta = delta_2 * 1

delta_1 = delta_2.dot(identity.T)
sox_delta = delta_1 * 1

delta_0 = delta_1.dot(identity.T)

alpha = 0.01
word_vects["red"] -= delta_0 * alpha
word_vects["sox"] -= sox_delta * alpha
word_vects["defeat"] -= defeat_delta * alpha

identity -= np.outer(layer_0, delta_1) * alpha
identity -= np.outer(layer_1, delta_2) * alpha
sent2output -= np.outer(layer_2, pred_delta) * alpha

print("==> sent2output:", sent2output)
print("==> identity:", identity)
