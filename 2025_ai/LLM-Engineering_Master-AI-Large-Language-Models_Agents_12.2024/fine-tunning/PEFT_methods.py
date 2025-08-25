#!/usr/bin/env python3

#### 1. Low-Rank_Adaptation
from peft import LoraConfig, get_peft_model

config = LoraConfig(
    r=8,  # 低秩维度
    lora_alpha=16,
    target_modules=["q_proj", "v_proj"],
    lora_dropout=0.1,
    bias="none"
)

model = get_peft_model(model, config)


#### 2. Adapter
#### 3. Prefix Tuning
#### 4. Supervised Fine-tuning, SFT
ds = {
  "instruction": "解释量子计算的基本概念",
  "input": "",
  "output": "量子计算是利用量子力学原理..."
}

#### 4. RLHF
training_args = TrainingArguments(
    output_dir="./results",
    per_device_train_batch_size=4,
    gradient_accumulation_steps=4,
    learning_rate=2e-5,
    num_train_epochs=3,
    fp16=True,
    save_steps=500,
    logging_steps=100,
)

rom transformers import AutoModelForCausalLM, AutoTokenizer, TrainingArguments, Trainer

model_name = "meta-llama/Llama-2-7b-hf"
tokenizer = AutoTokenizer.from_pretrained(model_name)
model = AutoModelForCausalLM.from_pretrained(model_name)

train_dataset = {}

# 训练参数
training_args = TrainingArguments(
    output_dir="./output",
    per_device_train_batch_size=4,
    num_train_epochs=3,
    learning_rate=2e-5,
    fp16=True,
    save_steps=500
)

trainer = Trainer(
    model=model,
    args=training_args,
    train_dataset=train_dataset,
    tokenizer=tokenizer
)

trainer.train()
