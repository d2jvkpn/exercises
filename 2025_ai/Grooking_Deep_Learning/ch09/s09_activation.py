#!/usr/bin/env python3

import os, argparse
from os import path
from sys import stdout, stderr
from datetime import datetime, timedelta

import yaml
import numpy as np
import polars as pl
from keras.datasets import mnist


parser = argparse.ArgumentParser(formatter_class=argparse.ArgumentDefaultsHelpFormatter)

#random_seed = 1
#batch_size = 100
#alpha = 5
#iterations = 1000  # 500, 1000, 2000, 5000
#hidden_size = 256# 128, 256, 512, 1024

parser.add_argument("--random_seed", type=int, default=1, help="random_seed")
parser.add_argument("--batch_size", type=int, default=100, help="batch_size")
parser.add_argument("--alpha", type=float, default=0.001, help="alpha") # ?? 5.0
parser.add_argument("--iterations", type=int, default=1000, help="iterations")
parser.add_argument("--hidden_size", type=int, default=256, help="hidden_size")
parser.add_argument("--train_size", type=int, default=1000, help="train_size")

args = parser.parse_args()

np.random.seed(args.random_seed)

# range=(-1.0, 1.0)
tanh = lambda x: np.tanh(x)
tanh2deriv = lambda y: 1.0 - y**2

# np.sum(y) = 1.0
def softmax(x):
     val = np.exp(x)
     return val / np.sum(val, axis=1, keepdims=True)

def one_hot_labels(labels): # [2] -> [[0, 0, 1, 0, 0, 0, 0, 0, 0, 0]]
    result = np.zeros((len(labels),10))

    for i, v in enumerate(labels):
        result[i][v] = 1

    return result

def format_timedelta(td: timedelta) -> str:
    sign, td = ("-", -td) if td < timedelta(0) else ("", td)
    hours, remainder = divmod(int(td.total_seconds()), 3600)
    minutes, seconds = divmod(remainder, 60)
    return f"{sign}{hours}:{minutes:02}:{seconds:02}.{td.microseconds:06}"

print()
print(f"==> 1. Parameters: args={args}")

#### 1. load data
(x_train, y_train), (x_test, y_test) = mnist.load_data() # ~/.keras/datasets/mnist.npz

args.train_size = min(args.train_size, len(x_train))
test_size = len(x_test)
image_shape = x_train[0].shape           # (28, 28)
pixels = image_shape[0] * image_shape[1] # 784

train_inputs = x_train[0:args.train_size].reshape(args.train_size, pixels) / 255 # shape=(N, 784)
train_labels = one_hot_labels(y_train[0:args.train_size])                        # shape(N, 10)

test_inputs = x_test[0:test_size].reshape(test_size, pixels) / 255 # shape=(N, 784)
test_labels = one_hot_labels(y_test[0:test_size])                  # shape(N, 10)

#### 2. Trainning the neural network
# range=(-0.01, 0.01), shape=(784, K)
weights_0_1 = np.random.random((pixels, args.hidden_size)) * 0.02 - 0.01

# range=(-0.1, 0.1), shape=(K, 10)
weights_1_2 = np.random.random((args.hidden_size, 10)) * 0.2 - 0.1

trainning_steps = []

t1 = datetime.now().astimezone()
print()
print(f"==> 2. Trainning: start_at={t1.isoformat('T')}")

for n in range(args.iterations):
    n += 1 # iteration number
    correct_cnt, train_error = 0, 0.0

    for i in range(int(args.train_size/args.batch_size)):
        batch_start, batch_end = i * args.batch_size, (i+1) * args.batch_size
        layer_0 = train_inputs[batch_start:batch_end] # shape=(batch_size, 784)
        target = train_labels[batch_start:batch_end]  # shape=(batch_size, 10)

        # 1. forward propagation
        layer_1 = tanh(np.dot(layer_0, weights_0_1)) # shape=(batch_size, K)
        # np.random.randint(2, size=layer_1.shape)
        dropout_mask = np.random.choice([0, 1], size=layer_1.shape, p=[0.5, 0.5])
        layer_1 *= (dropout_mask / 0.5)

        layer_2 = softmax(np.dot(layer_1, weights_1_2)) # shape=(batch_size, 10)

        # 2. backward propagation
        # delta_2 = (layer_2 - target) / (args.batch_size * layer_2.shape[0]) # ??
        delta_2 = layer_2 - target
        delta_1 = np.dot(delta_2, weights_1_2.T) * tanh2deriv(layer_1)
        delta_1 *= dropout_mask

        # no train_error here
        equals = np.argmax(layer_2, axis=1) == np.argmax(target, axis=1)
        correct_cnt += np.sum(equals.astype(int))

        # 3. update weights
        weights_1_2 -= np.dot(layer_1.T, delta_2) * args.alpha
        weights_0_1 -= np.dot(layer_0.T, delta_1) * args.alpha

    train_acc = correct_cnt/args.train_size

    if n%10 == 0 or n == args.iterations:
        layer_0, target = test_inputs, test_labels

        layer_1 = tanh(np.dot(layer_0, weights_0_1))
        layer_2 = np.dot(layer_1, weights_1_2)

        # no test_error here
        equals = np.argmax(layer_2, axis=1) == np.argmax(target, axis=1)
        correct_cnt = np.sum(equals.astype(int))
        test_acc = correct_cnt/test_size

        end_at = datetime.now().astimezone().isoformat('T')
        trainning_steps.append((n, train_acc, test_acc, end_at))
        print(f"--> I{n:04d}: train_accuracy={train_acc:.3f}", end=", ")
        print(f"test_accuracy={test_acc:.3f}, end_at={end_at}")

t2 = datetime.now().astimezone()

#### 3. Output the results
os.makedirs("data", mode=511, exist_ok=True)

wts_0_1 = pl.from_numpy(weights_0_1)
wts_1_2 = pl.from_numpy(weights_1_2)

trainning_steps = pl.from_records(
  trainning_steps,
  orient="row",
  schema=["iteration", "train_accuracy", "test_accuracy", "end_at"],
)

parameters = {
  "random_seed": args.random_seed,
  "batch_size": args.batch_size,
  "alpha": args.alpha,
  "iterations": args.iterations,
  "hidden_size": args.hidden_size,
  "activation_functions": ["tanh", "softmax"],

  "train_size": args.train_size,
  "test_size": test_size,
  "start_at": t1.isoformat('T'),
  "end_at": t2.isoformat('T'),
  "elapsed": format_timedelta(t2 - t1),
}

wts_0_1.write_csv(path.join("data", "wts_0_1.tsv"), separator='\t')
wts_1_2.write_csv(path.join("data", "wts_1_2.tsv"), separator='\t')
trainning_steps.write_csv(path.join("data", "trainning_steps.tsv"), separator='\t')

with open(path.join("data", "trainning_parameters.yaml"), 'w') as file:
    yaml.safe_dump(parameters, file, sort_keys=False)

print()
print(f"==> 3. Results:")
print(f"    weights_0_1={wts_0_1}")
print(f"    weights_1_2={wts_1_2}")

print()
print(f"<== 4. Exit: elapsed={t2 - t1}")
