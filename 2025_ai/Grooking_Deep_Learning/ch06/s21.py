#!/usr/bin/env python3

import numpy as np
np.random.seed(1)


#### 1. funcs
def relu(x: np.typing.ArrayLike): # Rectified Linear Unit
    return (x > 0).astype(float)  * x # numpy array

def relu2deriv(output: np.typing.ArrayLike):
    return (output > 0).astype(int) # numpy array

alpha = 0.2
hidden_size = 4
weights_0_1 = 2*np.random.random((3, hidden_size)) - 1.0 # range=(-1.0, 1.0)
weights_1_2 = 2*np.random.random((hidden_size, 1)) - 1.0 # range=(-1.0, 1.0)

# (6, 3)
streelights = np.array([
  [1, 0, 1],
  [0, 1, 1],
  [0, 0, 1],
  [1, 1, 1],
  [0, 1, 1],
  [1, 0, 1],
])

# (1, 6) -> (6, 1)
walk_vs_stop = np.array([[
  0,
  1,
  0,
  1,
  1,
  0,
]]).T

#layer_0 = streelights[0]
#layer_1 = relu(np.dot(layer_0, weights_0_1))
#layer_2 = relu(np.dot(layer_1, weights_1_2))

iterations = 60
size = streelights.shape[0]

#### 3. 
for n in range(iterations):
    n += 1
    error = 0.0

    for i in range(size):
        layer_0 =  streelights[i:i+1] # input layer, shape=(1, 3)
        target = walk_vs_stop[i:i+1]  # shape=(1, 1)

        layer_1 = relu(np.dot(layer_0, weights_0_1)) # hidden layer, shape=(1, 4)
        layer_2 = relu(np.dot(layer_1, weights_1_2)) # output layer, shape=(1, 1)

        #print("--- layer_0={}, layer_1={}, layer_2={}".format(
        #  np.round(layer_0, 3), np.round(layer_1, 3), np.round(layer_2, 3),
        #))

        delta_2 = layer_2 - target    # layer_2_delta, shape=(1, 1)
        error += np.sum(delta_2 ** 2) # layer_2_error

        delta_1 = delta_2.dot(weights_1_2.T) # layer_1_delta, (1,1) dot (1, 4) -> (1, 4)
        delta_1 *= relu2deriv(layer_1) # shape=(1, 4)

        # weights -= np.dot(output, delta) * alpha
        weights_0_1 -= layer_0.T.dot(delta_1) * alpha
        weights_1_2 -= layer_1.T.dot(delta_2) * alpha

    if n % 10 == 0:
        print(f"-> I{n:03d}: delta_2={np.round(delta_2, 6)}, error={error:.6f}")
        #print(f"    weights_0_1={weights_0_1}")
        #print(f"    weights_1_2={weights_1_2}")
