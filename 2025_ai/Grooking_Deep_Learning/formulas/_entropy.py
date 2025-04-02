#!/usr/bin/env python3

import numpy as np


np.set_printoptions(precision=3) # suppress=True, formatter={'float': '{: 0.3f}'.format}
y = np.array([1.5e-10, 1.5, 3.1415926, 1500])
print(y)

# data = np.random.rand(5)
d1 = np.array([0.2, 0.2, 0.2, 0.2, 0.2])
assert(d1.sum() == 1.0)
e1 = -d1 * np.log2(d1)

print("==> Entropy e1:", e1.sum())


#data = np.zeros(5)
# data = np.repeat(0.001, 5)
d2 = np.array([0.96, 0.01, 0.01, 0.01, 0.01])
assert(d2.sum() == 1.0)
e2 = -d2 * np.log2(d2)
print("==> Entropy e2:", e2.sum())


# Animal Labels: Dog, Fox, Horse, Eagle, Squirrel
target = np.eye(5)

p1 = target[0] # Dog, P1

q1 = np.array([0.4, 0.3, 0.05, 0.05, 0.2]) # model Q1
q2 = np.where(q1 == 0.0, 1e-9, q1)

q2 = np.array([0.98, 0.01, 0., 0., 0.01]) # model Q2
q2 = np.where(q2 == 0.0, 1e-9, q2)

hp1q1 = -p1*np.log2(q1) # H(P1, Q1)
hp1q2 = -p1*np.log2(q2) # H(P1, Q2)

print(f"""==> P1={p1}, Q1={q1}, Q2={q2}
    Cross Entropy: H(P1, Q1)={hp1q1.sum():.3f}
    Cross Entropy: H(P1, Q2)={hp1q2.sum():.3f}
""")
