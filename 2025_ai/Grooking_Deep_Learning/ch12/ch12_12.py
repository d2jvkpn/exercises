#!/usr/bin/env python3

import numpy as np

#### 1. identity matrix
this = np.array([2., 4, 6])
movie = np.array([10, 10, 10])
rocks = np.array([1, 1, 1])

identity = np.eye(3)

a = this + movie + rocks
b = (this.dot(identity) + movie).dot(identity) + rocks

print(f"this + movie + rocks = {a}")
print(f"(this.dot(identity) + movie).dot(identity) + rocks = {b}")

#### 2. init rnn
def softmax(x_):
    x = np.atleast_2d(x_)
    val = np.exp(x)
    return val / np.sum(val, axis=1, keepdims=True)

word_vects = {
  "yankees": np.array([[0., 0., 0.]]),
  "bears": np.array([[0., 0., 0.]]),
  "braves": np.array([[0., 0., 0.]]),
  "red": np.array([[0., 0., 0.]]),
  "sox": np.array([[0., 0., 0.]]),
  "lose": np.array([[0., 0., 0.]]),
  "defeat": np.array([[0., 0., 0.]]),
  "beat": np.array([[0., 0., 0.]]),
  "tie": np.array([[0., 0., 0.]]),
}

sent2output = np.random.rand(3, len(word_vects)) - 0.5
identity = np.eye(3)

#### 3. forward propagation
# red -> sox -> defeat -> yankees
layer_0 = word_vects["red"]
layer_1 = layer_0.dot(identity)  + word_vects["sox"]
layer_2 = layer_1.dot(identity) + word_vects["defeat"]

pred = softmax(layer_2.dot(sent2output)) # shape=(1, 9)
print(f"pred={pred}")

#### 4. back propagation
y = np.array([1., 0., 0., 0., 0., 0., 0., 0., 0.])

# yankees -> defeat -> sox -> red
pred_delta = pred - y                                      # yankees
delta_2 = pred_delta.dot(sent2output.T) * 1 # defeat
delta_1 = delta_2.dot(identity.T)  * 1              # sox
delta_0 = delta_1.dot(identity.T) * 1               # red

alpha = 0.01
word_vects["red"] -= delta_0 * alpha
word_vects["sox"] -= delta_1 * alpha
word_vects["defeat"] -= delta_2 * alpha

identity -= np.outer(layer_0, delta_1) * alpha
identity -= np.outer(layer_1, delta_2) * alpha
sent2output -= np.outer(layer_2,  pred_delta) * alpha
