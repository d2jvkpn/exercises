#!/usr/bin/env python3
import os
from pathlib import Path

import dotenv
dotenv.load_dotenv("configs/local.env")

import torch
from transformers import AutoTokenizer, AutoModel, AutoModelForCausalLM, BitsAndBytesConfig


model_id = "meta-llama/Llama-3.2-1B"

model_hf = Path(os.environ['HF_HOME']) / "hub" / ("models--" + model_id.replace("/", "--"))
model_ref = (model_hf / "refs" / "main").read_text(encoding="utf-8").strip()
model_path = model_hf / "snapshots" / model_ref

quant_config = BitsAndBytesConfig(
    load_in_4bit=True,
    bnb_4bit_compute_dtype=torch.bfloat16,

    bnb_4bit_use_double_quant=True,
    bnb_4bit_quant_type="nf4",
)
# quant_config = BitsAndBytesConfig(load_in_8bit=True, bnb_8bit_compute_dtype=torch.bfloat16)

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

output = '''
base_model: LlamaForCausalLM(
  (model): LlamaModel(
    (embed_tokens): Embedding(128256, 2048)
    (layers): ModuleList(
      (0-15): 16 x LlamaDecoderLayer(
        (self_attn): LlamaAttention(
          (q_proj): Linear4bit(in_features=2048, out_features=2048, bias=False)
          (k_proj): Linear4bit(in_features=2048, out_features=512, bias=False)
          (v_proj): Linear4bit(in_features=2048, out_features=512, bias=False)
          (o_proj): Linear4bit(in_features=2048, out_features=2048, bias=False)
        )
        (mlp): LlamaMLP(
          (gate_proj): Linear4bit(in_features=2048, out_features=8192, bias=False)
          (up_proj): Linear4bit(in_features=2048, out_features=8192, bias=False)
          (down_proj): Linear4bit(in_features=8192, out_features=2048, bias=False)
          (act_fn): SiLU()
        )
        (input_layernorm): LlamaRMSNorm((2048,), eps=1e-05)
        (post_attention_layernorm): LlamaRMSNorm((2048,), eps=1e-05)
      )
    )
    (norm): LlamaRMSNorm((2048,), eps=1e-05)
    (rotary_emb): LlamaRotaryEmbedding()
  )
  (lm_head): Linear(in_features=2048, out_features=128256, bias=False)
)
'''

print(f"Memory footprint: {base_model.get_memory_footprint() / 1e9:.1f} GB")

print("named_modules:", base_model.named_modules())
for name, module in base_model.named_modules():
    if any(x in name for x in ["proj", "fc"]):
        print(name)
