#!/usr/bin/env python3


delta = pred - y
error = delta ** 2
weights -= x * delta * alpha
