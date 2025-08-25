#!/usr/bin/env python3

from trl import SFTTrainer, SFTConfig
from datasets import Dataset


def to_messages(row):
    return {
        "messages": [
            {"role": "system", "content": "你是定价模型，只输出阿拉伯数字价格（元）。"},
            {"role": "user",   "content": f"为下述商品定价，并只输出数字：\n{row['desc']}"},
            {"role": "assistant", "content": f"{row['price']:.2f}"},
        ]
    }

raw = [{"desc":"100%棉短袖T恤，200g重克，简约圆领，均码。", "price": 49.0}, ...]
ds = Dataset.from_list([to_messages(v) for v in raw])

sft_config = SFTConfig(
    max_length=512,
    assistant_only_loss=True,   # 只训练助手机器人的那条消息（即价格）
    output_dir="./price-sft-chat",
)

trainer = SFTTrainer(
    model="Qwen/Qwen2-0.5B-Instruct",  # 选一个支持chat template的基座
    args=sft_config,
    train_dataset=ds,
)

trainer.train()
