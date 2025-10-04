#!/usr/bin/env python3
import os
from pathlib import Path
from dotenv import load_dotenv
load_dotenv(Path("configs") / "local.env", override=True)

from openai import OpenAI


####
openai = OpenAI(
    api_key=os.getenv('OPENAI_API_KEY'),
    base_url=os.getenv("OPENAI_API_BASE"),
)

####
print("--> 1")

response = openai.chat.completions.create(
    model="gpt-4.1-nano",
    messages=[{"role": "user", "content": "What is 2+2?"}],
)

print(response.choices[0].message.content)


####
msg = "Please propose a hard, challenging question to assess someone's IQ. Respond only with the question."
print(f"==> Question: {msg}")

####
print("--> 2")

response = openai.chat.completions.create(
    model="gpt-4.1-mini",
    messages=[{"role": "user", "content": msg }],
)

question = response.choices[0].message.content

print(question)

####
print("--> 3")
response = openai.chat.completions.create(
    model="gpt-4.1-mini",
    messages=[{"role": "user", "content": msg}],
)

answer = response.choices[0].message.content
print(answer)
