#!/usr/bin/env python3
import torch
torch.manual_seed(123)

class NeuralNetwork(torch.nn.Module):
    def __init__(self, num_inputs, num_outputs):
        super().__init__()

        self.layers = torch.nn.Sequential(
            # 1st hidden layer
            torch.nn.Linear(num_inputs, 30, bias=True),
            torch.nn.ReLU(),

            # 2nd hidden layer
            torch.nn.Linear(30, 20, bias=True),
            torch.nn.ReLU(),

            # output layer
            torch.nn.Linear(20, num_outputs, bias=True),
        )

    def forward(self, x):
        logits = self.layers(x)
        return logits

model = NeuralNetwork(50, 3)
print(model)

#### trainable parameters
num_params = sum(p.numel() for p in model.parameters() if p.requires_grad)
print("Total number of trainable model parameters:", num_params)

print(model.layers[0].weight)
print(model.layers[0].weight.shape)

print(model.layers[0].bias)
print(model.layers[0].bias.shape)

####
X = torch.rand((1, 50))
out = model(X)
print(out) # has grad_fn=<AddmmBackward0>
# Addmm stands for matrix multiplication (mm) followed by an addition (Add)

with torch.no_grad():
    out = model(X)
print(out) # no grad_fn

with torch.no_grad():
    out = torch.softmax(model(X), dim=1)
print(out)

####
X_train = torch.tensor([
    [-1.2, 3.1],
    [-0.9, 2.9],
    [-0.5, 2.6],
    [2.3, -1.1],
    [2.7, -1.5],
])

y_train = torch.tensor([0, 0, 0, 1, 1])

X_test = torch.tensor([
    [-0.8, 2.8],
    [2.6, -1.6],
])

y_test = torch.tensor([0, 1])
