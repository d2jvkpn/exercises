#!/usr/bin/env python3
from importlib.metadata import version

import tiktoken
import torch
torch.manual_seed(123)
from torch.utils.data import Dataset, DataLoader

print("torch version:", version("torch"))
print("tiktoken version:", version("tiktoken"))
 
class GPTDatasetV1(Dataset):
  def __init__(self, txt, tokenizer, max_length, stride):
    self.tokenizer = tokenizer
    self.input_ids = []
    self.target_ids = []

    token_ids = tokenizer.encode(txt, allowed_special={'<|endoftext|>'})

    for i in range(0, len(token_ids) - max_length, stride):
      input_chunk = token_ids[i:i + max_length]
      target_chunk = token_ids[i + 1: i + max_length + 1]
      self.input_ids.append(torch.tensor(input_chunk))
      self.target_ids.append(torch.tensor(target_chunk))

  def __len__(self):
    return len(self.input_ids)
 
  def __getitem__(self, idx): #D
    return self.input_ids[idx], self.target_ids[idx]
 
def create_dataloader(txt, batch_size=4, max_length=256, stride=128,
    shuffle=True, drop_last=True):
    # Initialize the tokenizer
    tokenizer = tiktoken.get_encoding("gpt2")

    # Create dataset
    dataset = GPTDatasetV1(txt, tokenizer, max_length, stride)

    # Create dataloader
    dataloader = DataLoader(dataset, batch_size=batch_size, shuffle=shuffle, drop_last=drop_last)

    return dataloader

####
vocab_size = 50257
output_dim = 256
context_length = 4
batch_size = 8

with open("../the-verdict.txt", "r", encoding="utf-8") as f:
    raw_text = f.read()

token_embedding_layer = torch.nn.Embedding(vocab_size, output_dim)
pos_embedding_layer = torch.nn.Embedding(context_length, output_dim)

dataloader = create_dataloader(
    raw_text,
    batch_size=batch_size,
    max_length=context_length,
    stride=context_length,
    shuffle=False,
)

data_iter = iter(dataloader)
inputs, targets = next(data_iter)

print(f"Input Tokens: {inputs.shape}\n{inputs}")

token_embeddings =  token_embedding_layer(inputs)
pos_embeddings = pos_embedding_layer(torch.arange(context_length))
print(pos_embeddings.shape)

input_embeddings = token_embeddings + pos_embeddings

assert(list(input_embeddings.shape) == [batch_size, context_length, output_dim])
