#! /bin/env python3

import tiktoken

tokenizer = tiktoken.get_encoding("gpt2")

####
text = "Hello, do you like tea? <|endoftext|> In the sunlit terraces of someunknownPlace."
print("~~~ text:", text)

integers = tokenizer.encode(text, allowed_special={"<|endoftext|>"})
print("~~~ integers:", integers)

strings = tokenizer.decode(integers)
print("~~~ strings:", strings)

####
with open("the-verdict.txt", "r", encoding="utf-8") as f:
  raw_text = f.read()

token_ids = tokenizer.encode(raw_text)
print("~~~ token_ids:", token_ids)
