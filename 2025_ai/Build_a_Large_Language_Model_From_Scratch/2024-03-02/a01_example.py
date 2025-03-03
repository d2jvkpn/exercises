#! /bin/env python3

import re

####
with open("the-verdict.txt", "r", encoding="utf-8") as f:
  raw_text = f.read()

# preprocessed = [item.strip() for item in preprocessed if item.strip()]
preprocessed = []

for t in re.split(r'([,.?_!"()\']|--|\s)', raw_text):
  t = t.strip()
  if t: preprocessed.append(t)


all_words = sorted(list(set(preprocessed)))
all_words.extend(["<|endoftext|>", "<|unk|>"])

vocab_size = len(all_words)

vocab = {token:integer for integer,token in enumerate(all_words)}

####
class SimpleTokenizer:
  decode_re = r'([,.?_!"()\']|--|\s)'
  encode_re = r'\s+([,.?!"()\'])'
  unknown = "<|unk|>"
  end_of_text = "<|endoftext|>"

  def __init__(self, vocab):
    self.str_to_int = vocab #A
    self.int_to_str = {i:s for s,i in vocab.items()} #B
    
  def encode(self, text): #C
    preprocessed = re.split(self.decode_re, text)
    preprocessed = [item.strip() for item in preprocessed if item.strip()]
    ids = [self.str_to_int.get(s, self.unknown)  for s in preprocessed]
    return ids
        
  def decode(self, ids): #D
    text = " ".join([self.int_to_str.get(i, self.unknown) for i in ids]) 
        
    text = re.sub(self.encode_re, r'\1', text) #E
    return text

####
tokenizer = SimpleTokenizer(vocab)
enc_text = tokenizer.encode(raw_text)
enc_sample = enc_text[50:]
context_size = 4

for i in range(1, context_size+1):
    context = enc_sample[:i]
    desired = enc_sample[i]

    print(
      "==> {} ---> {}\n    {} ---> {}".format(
        context, desired,
        tokenizer.decode(context), tokenizer.decode([desired]),
      )
    )
