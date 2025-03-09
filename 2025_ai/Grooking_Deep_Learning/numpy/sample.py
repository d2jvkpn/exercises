#!/usr/bin/env python3

import numpy as np


A = np.array(["apple", "banana", "cherry", "date"])
B = np.array([10, 20, 30, 40]) 

probabilities = B / np.sum(B)

sample_size = 5
sampled_elements = np.random.choice(A, size=sample_size, p=probabilities, replace=False)

print("抽样结果:", sampled_elements)
