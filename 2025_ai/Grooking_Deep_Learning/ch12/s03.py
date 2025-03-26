#!/usr/bin/env python3

import numpy as np
np.random.seed(1)


####
a = np.array([1, 2, 3])
b = np.array([0.1, 0.2, 0.3])
c = np.array([-1, -0.5, 0])
d = np.array([0, 0, 0])

identity = np.eye(3)
print()
print("==> 1. identity", identity)
print(f"   {{a, b, c, d}}: {{{a}, {b}, {c}, {d}}}")

print()
print("==> 2. dot identity")
print("a.dot(identity):", np.dot(a, identity))
print("b.dot(identity):", np.dot(b, identity))
print("c.dot(identity):", np.dot(c, identity))
print("d.dot(identity):", np.dot(d, identity))

####
this = np.array([2, 4,6])
movie = np.array([10, 10, 10])
rocks = np.array([1, 1, 1])

print()
print("==> 3. this + movie + rocks")
print("this + movie + rocks:", this + movie + rocks)

print(
  "(this.dot(identity) + movie).dot(identity) + rocks:",
  (this.dot(identity) + movie).dot(identity) + rocks,
)
