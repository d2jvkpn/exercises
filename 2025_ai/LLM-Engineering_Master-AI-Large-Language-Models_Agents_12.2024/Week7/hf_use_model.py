#!/usr/bin/env python3
import os
from pathlib import Path
import dotenv
dotenv.load_dotenv("configs/local.env")

import torch
from transformers import AutoTokenizer, AutoModel, AutoModelForCausalLM, BitsAndBytesConfig
from transformers import pipeline


def model_abspath(model_id):
    cache_dir = os.environ.get('HF_HUB_CACHE')
    if cache_dir is None:
        cache_dir = os.path.join(os.environ["HOME"], ".cache", "huggingface", "hub")

    model_hf = Path(cache_dir) / ("models--" + model_id.replace("/", "--"))
    model_ref = (model_hf / "refs" / "main").read_text(encoding="utf-8").strip()
    model_path = model_hf / "snapshots" / model_ref

    return model_path

####
model_id = "meta-llama/Llama-3.2-1B"
model_path = model_abspath(model_id)

quant_config = BitsAndBytesConfig(
    load_in_4bit=True,
    bnb_4bit_compute_dtype=torch.bfloat16,

    bnb_4bit_use_double_quant=True,
    bnb_4bit_quant_type="nf4",
)
# quant_config = BitsAndBytesConfig(load_in_8bit=True, bnb_8bit_compute_dtype=torch.bfloat16)

####
tokenizer = AutoTokenizer.from_pretrained(model_path, trust_remote_code=True)
tokenizer.pad_token = tokenizer.eos_token
tokenizer.padding_side = "right"

base_model = AutoModelForCausalLM.from_pretrained(
    model_path,
    quantization_config=quant_config,
    device_map="auto",
)

base_model.generation_config.pad_token_id = tokenizer.pad_token_id
print(f"base_model: {base_model}")

print(f"Memory footprint: {base_model.get_memory_footprint() / 1e9:.1f} GB")

print("named_modules:", list(base_model.named_modules()))
for name, module in base_model.named_modules():
    if any(x in name for x in ["proj", "fc"]):
        print(name)

####
os.sys.exit(0)
q4_dir = "data/Llama-3.2-1B-q4"

tokenizer.save_pretrained(q4_dir)
base_model.save_pretrained(q4_dir)

del tokenizer, base_model
torch.cuda.empty_cache()

generator = pipeline("text-generation", model=q4_dir)

response = generator("Hello, world!", max_new_tokens=42, truncation=True)
