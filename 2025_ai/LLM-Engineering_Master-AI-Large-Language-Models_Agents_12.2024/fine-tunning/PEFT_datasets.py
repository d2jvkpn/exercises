#!/usr/bin/env python3

#### 1. text for writing
train_dataset = [
    {"text": "深度学习是机器学习的一个分支..."},
    {"text": "巴黎是法国的首都，以埃菲尔..."},
]

#### 2. Instruction Fine-tuning
train_dataset = [
    {
        "instruction": "将以下句子翻译成英文",
        "input": "今天天气真好",
        "output": "The weather is nice today",
    },
    {
        "instruction": "概括这段文字",
        "input": "深度学习需要大量数据...",
        "output": "深度学习依赖大数据",
    }
]

def format_instruction(example):
    return f"### Instruction:\n{example['instruction']}\n\n### Input:\n{example['input']}\n\n### Response:\n{example['output']}"

dataset = dataset.map(format_instruction)


#### 3. Chat Model
train_dataset = [
    {
        "conversations": [
            {"role": "user", "content": "你好！"},
            {"role": "assistant", "content": "你好，有什么可以帮您？"},
            {"role": "user", "content": "推荐一本好书"},
            {"role": "assistant", "content": "《人类简史》值得一读"},
        ]
    }
]

#### 4. classification
from datasets import load_dataset
from torch.utils.data import Dataset

train_dataset = [
    {"text": "这个产品太糟糕了", "label": "negative"},
    {"text": "非常满意的购物体验", "label": "positive"},
]

dataset = load_dataset('json', data_files='train.jsonl')
#dataset = load_dataset('csv', data_files='train.csv')
#dataset = load_dataset('imdb')


class CustomDataset(Dataset):
    def __init__(self, texts, labels, tokenizer, max_length):
        self.encodings = tokenizer(texts, truncation=True, padding='max_length', max_length=max_length)
        self.labels = labels
    
    def __getitem__(self, idx):
        return {
            'input_ids': torch.tensor(self.encodings['input_ids'][idx]),
            'attention_mask': torch.tensor(self.encodings['attention_mask'][idx]),
            'labels': torch.tensor(self.labels[idx])
        }


tokenizer(text, truncation=True, padding='max_length', max_length=512)

def data_collator(features):
    batch = tokenizer.pad(
        features,
        padding='longest',
        return_tensors='pt',
    )
    batch['labels'] = batch['input_ids'].clone()

    return batch

dataset = dataset.train_test_split(test_size=0.1)

tokenizer.add_special_tokens({'additional_special_tokens': ['<|user|>', '<|assistant|>']})

#### Alpaca dataset
dataset = [
    {
        "instruction": "给出三个保持健康的建议",
        "input": "",
        "output": "1. 均衡饮食\n2. 定期锻炼\n3. 充足睡眠",
    }
]

def preprocess_function(examples):
    inputs = [f"Instruction: {i}\nInput: {inp}\nOutput: " 
              for i, inp in zip(examples['instruction'], examples['input'])]
    model_inputs = tokenizer(inputs, truncation=True, max_length=512)
    
    outputs = tokenizer(examples['output'], truncation=True, max_length=512)
    model_inputs["labels"] = outputs["input_ids"]
    return model_inputs

dataset = dataset.map(preprocess_function, batched=True)
