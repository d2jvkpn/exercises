#!/usr/bin/env python3
import torch
import torch.nn.functional as F


x1 = torch.tensor([1.1]) # Input feature
y = torch.tensor([1.0])  # True label

w1 = torch.tensor([2.2]) # Weight parameter
b = torch.tensor([0.0])  # Bias unit
z = x1 * w1 + b          # Net input

a = torch.sigmoid(z)    # Activation and output

loss = F.binary_cross_entropy(a, y)

#A logistic regression forward pass as a computation graph. The input feature
#x1 is multiplied by a model weight w1 and passed through an activation function σ after
#adding the bias. The loss is computed by comparing the model output a with a given label y.

#The most common way of computing the loss gradients in a
#computation graph involves applying the chain rule from right to left, also called
#reverse-model automatic differentiation or backpropagation. We start from the
#output layer (or the loss itself) and work backward through the network to the input
#layer. We do this to compute the gradient of the loss with respect to each parameter
#(weights and biases) in the network, which informs how we update these
#parameters during training.
