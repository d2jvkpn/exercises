#!/bin/bash
set -eu -o pipefail; _wd=$(pwd); _dir=$(readlink -f `dirname "$0"`)

tensorboard --logdir=./logs

exit

from transformers import TrainingArguments
from trl import SFTTrainer
import logging

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
