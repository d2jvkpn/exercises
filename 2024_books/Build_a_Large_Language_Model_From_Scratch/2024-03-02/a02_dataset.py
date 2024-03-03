#! /bin/env python3

import torch
from torch.utils.data import Dataset, DataLoader
 
class GPTDatasetV1(Dataset):
  def __init__(self, txt, tokenizer, max_length, stride):
    self.tokenizer = tokenizer
    self.input_ids = []
    self.target_ids = []

    token_ids = tokenizer.encode(txt) #A
 
    for i in range(0, len(token_ids) - max_length, stride): #B
      input_chunk = token_ids[i:i + max_length]
      target_chunk = token_ids[i + 1: i + max_length + 1]
      self.input_ids.append(torch.tensor(input_chunk))
      self.target_ids.append(torch.tensor(target_chunk))

  def __len__(self): #C
    return len(self.input_ids)
 
  def __getitem__(self, idx): #D
    return self.input_ids.get(idx), self.target_ids.get(idx)


def create_dataloader(txt, batch_size=4, max_length=256, stride=128):
  tokenizer = tiktoken.get_encoding("gpt2") #A 
  dataset = GPTDatasetV1(txt, tokenizer, max_length, stride) #B
  dataloader = DataLoader(dataset, batch_size=batch_size) #C
  return dataloader

with open("the-verdict.txt", "r", encoding="utf-8") as f:
  raw_text = f.read()

#### 
dataloader = create_dataloader(raw_text, batch_size=1, max_length=4, stride=1)
data_iter = iter(dataloader) #A
first_batch = next(data_iter)
second_batch = next(data_iter)

print(first_batch, second_batch)

####
dataloader = create_dataloader(raw_text, batch_size=8, max_length=4, stride=5)
 
data_iter = iter(dataloader)
inputs, targets = next(data_iter)
