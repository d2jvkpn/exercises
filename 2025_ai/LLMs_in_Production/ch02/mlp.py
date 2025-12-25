#!/usr/bin/env python3
import torch
import torch.nn as nn
import torch.nn.functional as F


class MultiLayerPerceptron(nn.Module):
    def __init__(
        self,
        input_size,
        hidden_size=2,
        output_size=3,
        num_hidden_layers=1,
        hidden_activation=nn.Sigmoid,
    ):
        """Initialize weights.
        Args:
            input_size (int): size of the input
            hidden_size (int): size of the hidden layers
            output_size (int): size of the output
            num_hidden_layers (int): number of hidden layers
            hidden_activation (torch.nn.*): the activation class
        """
        super(MultiLayerPerceptron, self).__init__()
        torch.device("cuda:0" if torch.cuda.is_available() else "cpu")

        self.module_list = nn.ModuleList()

        self.module_list.append(nn.Linear(input_size, hidden_size))
        for _ in range(num_hidden_layers - 1):
            self.module_list.append(nn.Linear(hidden_size, hidden_size))
            self.module_list.append(hidden_activation())
        self.fc_final = nn.Linear(hidden_size, output_size)

        self.last_forward_cache = []

    def forward(self, x, apply_softmax=False):
        """The forward pass of the MLP

        Args:
            x_in (torch.Tensor): an input data tensor.
                x_in.shape should be (batch, input_dim)
            apply_softmax (bool): a flag for the softmax activation
                should be false if used with the Cross Entropy losses
        Returns:
            the resulting tensor. tensor.shape should be (batch, output_dim)
        """
        for module in self.module_list:
            x = module(x)

        output = self.fc_final(x)

        if apply_softmax:
            output = F.softmax(output, dim=1)

        return output
