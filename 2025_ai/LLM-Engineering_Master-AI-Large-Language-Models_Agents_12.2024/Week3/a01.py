#!/usr/bin/env python3


####
quant_config = BitsAndBytesConfig(load_in_8bit=True)

tokenizer = AutoTokenizer.from_pretrained(model_id)
model = AutoModelForCausalLM.from_pretrained(
    model_id, quantization_config=quant_config, device_map="auto",
)

streamer = TextStreamer(tokenizer)

inputs = tokenizer("你是谁？", return_tensors="pt").to(model.device)
model.generate(**inputs, streamer=streamer, max_new_tokens=100)



####
# pip install ctransformers
from ctransformers import AutoModelForCausalLM

model = AutoModelForCausalLM.from_pretrained(
    "TheBloke/Mistral-7B-Instruct-v0.2-GGUF",
    model_file="mistral-7b-instruct-v0.2.Q4_K_M.gguf",  # 你下载的 gguf 模型文件
    model_type="mistral",
    max_new_tokens=100
)
