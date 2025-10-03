#!/usr/bin/env python3
from pathlib import Path

#from data.minbpe import BasicTokenizer as Tokenizer
from src.minbpe import RegexTokenizer as Tokenizer
#from src.gpt import GPTLanguageModel
from src.transformer.model_relative_positional_encoding import GPTLanguageModel

import torch
torch.manual_seed(3647)
torch.set_float32_matmul_precision('high')
import torch.nn as nn


def get_model_footprint(model: nn.Module):
    param_size = 0
    buffer_size = 0

    for param in model.parameters():
        param_size += param.nelement() * param.element_size()

    for buffer in model.buffers():
        buffer_size += buffer.nelement() * buffer.element_size()

    size_all_mb = (param_size + buffer_size) / 1024**2

    return size_all_mb

#### 1.
device = 'cuda' if torch.cuda.is_available() else 'cpu'

tokenizer_dir = Path("data") / "tokenizer"
checkpoint_dir = Path("data") / "ch11"
#checkpoint_dir.mkdir(parents=True, exist_ok=True)

#### 2.
tokenizer = Tokenizer()
tokenizer.load(model_file=str(tokenizer_dir / "tokenizer.model"))

#### 3.
ckpt_files = sorted(
    checkpoint_dir.glob("checkpoint_*.pt"),
    #key=lambda x: x.stat().st_ctime,
    key=lambda x: int(x.name.split("-")[1].replace(".pt", "")),
    reverse=True,
)

last_ckpt = torch.load(ckpt_files[0], map_location=device)
parameters = last_ckpt['meta']['parameters']

model = GPTLanguageModel(
    vocab_size=parameters['vocab_size'],
    n_embd=parameters['n_embd'],
    block_size=parameters['block_size'],
    n_head=parameters['n_head'],
    n_layer=parameters['n_layer'],
    dropout=parameters['dropout'],
    device=device,
).to(device)

model = torch.compile(model)

model.load_state_dict(last_ckpt['model_state_dict'])

#### 4.
@torch.no_grad()
def llm(msg, max_new_tokens=100):
    input_tokens = tokenizer.encode(msg, allowed_special="all")
    input_tokens = torch.tensor(input_tokens, dtype=torch.long).unsqueeze(0).to(device)

    # with torch.no_grad():
    output_tokens = model.advanced_generation(
        input_tokens=input_tokens, max_new_tokens=max_new_tokens,
        temperature=0.9, top_k=50, top_p=None,
    )

    output_tokens = output_tokens[0].tolist()

    return tokenizer.decode(output_tokens)


msg = "What's GTP and LLM?"
answer = llm(msg)

print(answer)
