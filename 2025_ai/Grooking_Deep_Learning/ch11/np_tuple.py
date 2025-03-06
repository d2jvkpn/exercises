#!/usr/bin/env python3

import numpy as np

dtype = [('name', 'U10'), ('value', 'i4')]

arr = np.array([("hello", 42), ("world", 99)], dtype=dtype)
print(arr)
