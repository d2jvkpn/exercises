#! /bin/env python3

import re

with open("the-verdict.txt", "r", encoding="utf-8") as f:
  raw_text = f.read()

print("==> Total number of character:", len(raw_text))
print("~~ raw_text[:99]:", raw_text[:99])

# preprocessed = [item.strip() for item in preprocessed if item.strip()]
preprocessed = []

for t in re.split(r'([,.?_!"()\']|--|\s)', raw_text):
  t = t.strip()
  if t: preprocessed.append(t)

# print(preprocessed)


all_words = sorted(list(set(preprocessed)))
vocab_size = len(all_words)

print(f"==> vocab_size: {vocab_size}")
vocab = {token:integer for integer,token in enumerate(all_words)}

class SimpleTokenizer:
  decode_re = r'([,.?_!"()\']|--|\s)'
  encode_re = r'\s+([,.?!"()\'])'

  def __init__(self, vocab):
    self.str_to_int = vocab #A
    self.int_to_str = {i:s for s,i in vocab.items()} #B
    
  def encode(self, text): #C
    preprocessed = re.split(self.decode_re, text)
    preprocessed = [item.strip() for item in preprocessed if item.strip()]
    ids = [self.str_to_int[s] for s in preprocessed]
    return ids
        
  def decode(self, ids): #D
    text = " ".join([self.int_to_str[i] for i in ids]) 
        
    text = re.sub(self.encode_re, r'\1', text) #E
    return text

tokenizer = SimpleTokenizer(vocab)

text = """"It's the last he painted, you know," Mrs. Gisburn said with pardonable pride."""
ids = tokenizer.encode(text)

print("==> ids:", ids)
print("==> text:", tokenizer.decode(ids))
