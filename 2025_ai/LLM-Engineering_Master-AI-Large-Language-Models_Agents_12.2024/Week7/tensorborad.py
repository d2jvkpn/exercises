#!/usr/bin/env python3
import logging

from transformers import TrainingArguments
from trl import SFTTrainer


logging.basicConfig(
    filename="./logs/training.log",
    level=logging.INFO,
    format="%(asctime)s - %(levelname)s - %(message)s",
)


training_args = TrainingArguments(
    output_dir="./output",
    logging_dir="./logs",
    logging_strategy="steps",
    logging_steps=100,
    report_to=["tensorboard"],
)

trainer = SFTTrainer(
    model=model,
    args=training_args,
    train_dataset=train_dataset,
    eval_dataset=eval_dataset,
)


trainer.train()
