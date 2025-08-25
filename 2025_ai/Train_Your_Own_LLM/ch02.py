#!/usr/bin/env python3
from pathlib import Path
from typing import Optional, Tuple

from src.minbpe import BasicTokenizer
from src.gpt import GPTLanguageModel

import torch
torch.manual_seed(3647)
import torch.nn as nn
from torch.nn import functional as F
from torch.utils.data import Dataset, DataLoader


####
def print_model_structure(model: nn.Module, indent: str = '') -> None:
    """
    Custom function to print model structure in a hierarchical format
    """
    for name, child in model.named_children():
        params = sum(p.numel() for p in child.parameters())
        print(f"{indent}├─ {name}: {child.__class__.__name__} ({params:,} parameters)")
        print_model_structure(child, indent + '│  ')

class TextDataset(Dataset):
    def __init__(self, data: torch.Tensor, block_size: int) -> None:
        self.data = data
        self.block_size = block_size

    def __len__(self) -> int:
        return len(self.data) - self.block_size

    def __getitem__(self, index: int) -> Tuple[torch.Tensor, torch.Tensor]:
        x = self.data[index:index + self.block_size]
        y = self.data[index + 1:index + self.block_size + 1]
        return x, y


def get_dataloaders(
        train_data: torch.Tensor,
        val_data: torch.Tensor,
        block_size: int,
        batch_size: int,
        device: torch.device
) -> Tuple[DataLoader, DataLoader]:
    train_dataset = TextDataset(train_data.to(device), block_size)
    val_dataset = TextDataset(val_data.to(device), block_size)

    train_loader = DataLoader(
        train_dataset,
        batch_size=batch_size,
        shuffle=True,
    )
    val_loader = DataLoader(
        val_dataset,
        batch_size=batch_size,
        shuffle=False,
    )

    return train_loader, val_loader


####
output_dir = Path("data") / "tokenizer"

tokenizer = BasicTokenizer()
tokenizer.load(model_file=str(output_dir / "my_tokenizer.model"))

with open(output_dir / "train.txt", 'r') as f:
    text = f.read()
    train_data = tokenizer.encode(text)

with open(output_dir / "validation.txt", 'r') as f:
    text = f.read()
    val_data = tokenizer.encode(text)


####
block_size = 256
n_embd = 512
n_head = 8
n_layer = 4
dropout = 0.2
batch_size = 64
vocab_size = len(tokenizer.vocab) + len(tokenizer.special_tokens)
device = 'cuda' if torch.cuda.is_available() else 'cpu'

model = GPTLanguageModel(
    vocab_size=vocab_size,
    block_size=block_size,
    n_embd=n_embd,
    n_head=n_head,
    n_layer=n_layer,
    dropout=dropout,
    device=device,
).to(device)

model = torch.compile(model)

num_parameters = sum(p.numel() for p in model.parameters()) / 1e6
print(f'--> {num_parameters:.3}M parameters')

print_model_structure(model)
