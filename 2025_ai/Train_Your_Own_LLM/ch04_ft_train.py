#!/usr/bin/env python3
from pathlib import Path
from typing import Tuple

from src.minbpe import RegexTokenizer
from src.gpt import GPTLanguageModel

import torch
from torch.utils.data import Dataset, DataLoader


class FineTuningDataset(Dataset):
    def __init__(self, data: torch.Tensor, device: torch.device, padding_token: int):
        self.data = data
        self.device = device
        self.padding_token = padding_token

    def __len__(self) -> int:
        return len(self.data)

    def __getitem__(self, index: int) -> Tuple[torch.Tensor, torch.Tensor]:
        sample = self.data[index]
        x = sample.to(self.device)
        y = sample[1:].to(self.device)
        padding_tensor = torch.tensor([self.padding_token], device=self.device)
        y = torch.cat(y, padding_tensor)

        return x, y


#### 1.
tokenizer_dir = Path("data") / "tokenizer"
tokenizer = RegexTokenizer()
tokenizer.load(model_file=str(tokenizer_dir / "tokenizer.model"))

padding_token = -100
batch_size = 64
device = torch.device("cuda" if torch.cuda.is_available() else "cpu")

block_size = 256
n_embd = 512
n_head = 8
n_layer = 4
dropout = 0.2
vocab_size = len(tokenizer.vocab)

model = GPTLanguageModel(
    vocab_size=vocab_size,
    block_size=block_size,
    n_embd=n_embd,
    n_head=n_head,
    n_layer=n_layer,
    dropout=dropout,
    device = device,
)

model = torch.compile(model)
checkpoint_path = Path("data") / "ch02_checkpoints" / "checkpoint_001-007200.pt"
checkpoint = torch.load(checkpoint_path, weights_only=True, map_location=device)
model.load_state_dict(checkpoint["model_state_dict"])

input_tokens = tokenizer.encode("hello, world", allowed_special="all")
input_tokens = torch.tensor(input_tokens, dtype=torch.long).unsqueeze(0).to(device)

model.eval()
with torch.no_grad():
    output = model.generate(input_tokens=input_tokens, max_new_tokens=100)
    print(tokenizer.decode(output[0].tolist()))

#### 2.
train_tensor = torch.load(tokenizer_dir /'train.fine-tuning.pt')
val_tensor = torch.load(tokenizer_dir /'validation.fine-tuning.pt')

train_dataset = FineTuningDataset(data=train_tensor, device=device, padding_token=padding_token)
train_loader = DataLoader(dataset=train_dataset, batch_size=batch_size)

val_dataset = FineTuningDataset(data=val_tensor, device=device, padding_token=padding_token)
val_loader = DataLoader(dataset=val_dataset, batch_size=batch_size)



