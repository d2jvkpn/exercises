#!/usr/bin/env python3

import modal

# Setup - define our infrastructure with code!
app = modal.App("pricer-service")
image = modal.Image.debian_slim()
    .pip_install("huggingface", "torch", "transformers", "bitsandbytes", "accelerate", "peft")

# This collects the secret from Modal.
# Depending on your Modal configuration, you may need to replace "hf-secret" with "huggingface-secret"
secrets = [modal.Secret.from_name("hf-secret")]

hf_cache_volume = modal.Volume.from_name("hf-hub-cache", create_if_missing=True)

# Constants
BASE_MODEL = "meta-llama/Meta-Llama-3.1-8B"
HF_USER = "ed-donner" # your HF name here! Or use mine if you just want to reproduce my results.
FINETUNED_MODEL = f"{HF_USER}/pricer-2024-09-13_13.04.39"
REVISION = "e8d637df551603dc86cd7a1598a8f44af4d7ae36"

QUESTION = "How much does this cost to the nearest dollar?"
PREFIX = "Price is $"


@app.cls(
    image=image.env({
        "HF_HUB_CACHE": "data/huggingface/hub",
        "HF_DATASETS_CACHE": "data/huggingface/datasets",
    }),
    secrets=secrets, 
    volumes={"/root/data/huggingface": hf_cache_volume}
    # Change this to 1 if you want Modal to be always running, otherwise it will go cold after 2 mins
    min_containers=0,
    max_containers=1,
    gpu="T4", 
    timeout=1800,
)
class Pricer:
    @modal.enter()
    def setup(self):
        import torch
        from transformers import AutoTokenizer, AutoModelForCausalLM, BitsAndBytesConfig, set_seed
        from peft import PeftModel
        
        # Quant Config
        quant_config = BitsAndBytesConfig(
            load_in_4bit=True,
            bnb_4bit_use_double_quant=True,
            bnb_4bit_compute_dtype=torch.bfloat16,
            bnb_4bit_quant_type="nf4",
        )

        # Load model and tokenizer
        self.tokenizer = AutoTokenizer.from_pretrained(BASE_MODEL)
        self.tokenizer.pad_token = self.tokenizer.eos_token
        self.tokenizer.padding_side = "right"

        self.base_model = AutoModelForCausalLM.from_pretrained(
            BASE_MODEL, 
            quantization_config=quant_config,
            device_map="auto",
        )

        self.fine_tuned_model = PeftModel.from_pretrained(
            self.base_model, FINETUNED_MODEL, revision=REVISION,
        )

    @modal.method()
    def price(self, description: str) -> float:
        import os, re

        import torch
        from transformers import AutoTokenizer, AutoModelForCausalLM, BitsAndBytesConfig, set_seed
        from peft import PeftModel
    
        set_seed(42)
        prompt = f"{QUESTION}\n\n{description}\n\n{PREFIX}"

        inputs = self.tokenizer.encode(prompt, return_tensors="pt").to("cuda")
        attention_mask = torch.ones(inputs.shape, device="cuda")

        outputs = self.fine_tuned_model.generate(
            inputs, attention_mask=attention_mask,
            max_new_tokens=5, num_return_sequences=1,
        )

        result = self.tokenizer.decode(outputs[0])
    
        contents = result.split("Price is $")[1]
        contents = contents.replace(',','')
        match = re.search(r"[-+]?\d*\.\d+|\d+", contents)

        return float(match.group()) if match else 0
