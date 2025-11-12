#!/usr/bin/env python3


val = np.exp(x)
softmax =  val / np.sum(val, axis=1, keepdims=True)
