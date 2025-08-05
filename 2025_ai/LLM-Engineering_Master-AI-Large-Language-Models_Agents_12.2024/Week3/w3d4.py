#!/usr/bin/env python3
#from pathlib import Path
import importlib

# $ pip install bitsandbytes sentencepiece
#from dotenv import load_dotenv

from google.colab import userdata
from huggingface_hub import login
import torch
from transformers import AutoTokenizer, AutoModelForCausalLM, TextStreamer, BitsAndBytesConfig

#### 1. check
from transformers.utils import is_bitsandbytes_available
assert(is_bitsandbytes_available())
assert(importlib.util.find_spec("bitsandbytes") is not None)
print(f"bitsandbytes: {importlib.metadata.version('bitsandbytes')}")

#### 2. setup
#load_dotenv(dotenv_path=Path("configs") / "local.env")
hf_token = userdata.get('HF_TOKEN')
login(hf_token, add_to_git_credential=True)

#model_id = "data/models/google/gemma-3-4b-it.model"
model_id = "google/gemma-3-4b-it.model"

#### 3. model
quant_config = BitsAndBytesConfig (
    load_in_4bit=True,
    bnb_4bit_use_double_quant=True,
    bnb_4bit_compute_dtype=torch.bfloat16,
    bnb_4bit_quant_type="nf4",
)


tokenizer = AutoTokenizer.from_pretrained(model_id) # trust_remote_code=True
print(len(tokenizer.vocab), tokenizer.vocab_size, tokenizer.model_max_length)
# 32768 32768 1000000000000000019884624838656
tokenizer.pad_token = tokenizer.eos_token

model = AutoModelForCausalLM.from_pretrained(
    model_id, device_map="auto",
    quantization_config=quant_config,
)


memory = model.get_memory_footprint()/1e6
print(f"memory: {memory}")

### 4. 
streamer = TextStreamer(tokenizer)

messages = [
    {"role": "system", "content": "You are a helpful assitant."},
    {"role": "user", "content": "Tell me a light-hearted joke from a room of Data Scientists."},
]

inputs = tokenizer.apply_chat_template(messages, return_tensors="pt").to("cuda")

outputs = model.generate(inputs, max_new_tokens=80)
print(tokenizer.decode(outputs[0]))


#####
model

del inputs, outputs, model

torch..cuda.empty_cache()


####
def generate(model, messages):
    tokenizer = AutoTokenizer.from_pretrained(model)
    tokenizer.pad_token = tokenizer.eos_token
    inputs = tokenizer.apply_chat_template(messages, return_tensors="pt").to("cuda")

    model = AutoModelForCausalLM.from_pretrained(
        model, device_map="auto", quantization_config=quant_config,
    )

    outputs = model.generate(inputs, max_new_tokens=80, streamer=streamer)
    del tokenizer, streamer, model, inputs, outputs
    torch.cuda.empty_cache()
