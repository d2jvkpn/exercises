#! /bin/env python3

import re

####
with open("the-verdict.txt", "r", encoding="utf-8") as f:
  raw_text = f.read()

print("==> Total number of character:", len(raw_text))
print("~~ raw_text[:99]:", raw_text[:99])

####
text = "Hello, world. This, is a test."

print(re.split(r'(\s)', text))

result = re.split(r'([,.]|\s)', text)
result = [item.strip() for item in result if item.strip()]
print(result)

text = "Hello, world. Is this-- a test?"
result = re.split(r'([,.?_!"()\']|--|\s)', text)
result = [item.strip() for item in result if item.strip()]
print(result)
