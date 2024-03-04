#! /bin/env python3

import torch
from torch.utils.data import Dataset, DataLoader
import tiktoken

from importlib.metadata import version

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


def create_dataloader(txt, batch_size=4, max_length=256, stride=128):
  tokenizer = tiktoken.get_encoding("gpt2")
  dataset = GPTDatasetV1(txt, tokenizer, max_length, stride)
  dataloader = DataLoader(dataset, batch_size=batch_size)
  return dataloader

def create_dataloader(txt, batch_size=4, max_length=256, stride=128,
  shuffle=True, drop_last=True):
  # Initialize the tokenizer
  tokenizer = tiktoken.get_encoding("gpt2")

  # Create dataset
  dataset = GPTDatasetV1(txt, tokenizer, max_length, stride)

  # Create dataloader
  dataloader = DataLoader(dataset, batch_size=batch_size, shuffle=shuffle, drop_last=drop_last)

  return dataloader

with open("the-verdict.txt", "r", encoding="utf-8") as f:
  raw_text = f.read()

####
tokenizer = tiktoken.get_encoding("gpt2")
encoded_text = tokenizer.encode(raw_text)

vocab_size, output_dim, block_size = 50257, 256, 1024


####
dataloader = create_dataloader(raw_text, batch_size=8, max_length=max_length, stride=max_length)

# data_iter = iter(dataloader)
# first_batch = next(data_iter)
# second_batch = next(data_iter)

token_embedding_layer = torch.nn.Embedding(vocab_size, output_dim)
pos_embedding_layer = torch.nn.Embedding(block_size, output_dim)

for batch in dataloader:
  x, y = batch

  token_embeddings = token_embedding_layer(x)
  pos_embeddings = pos_embedding_layer(torch.arange(max_length))
  input_embeddings = token_embeddings + pos_embeddings
  break

print(x, y)

####
dataloader = create_dataloader(raw_text, batch_size=1, max_length=max_length, stride=1)
