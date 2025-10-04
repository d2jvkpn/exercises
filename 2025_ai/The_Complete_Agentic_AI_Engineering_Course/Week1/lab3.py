#!/usr/bin/env python3
import os, json
from pathlib import Path
from dotenv import load_dotenv
load_dotenv(Path("configs") / "local.env", override=True)

from openai import OpenAI
from pypdf import PdfReader
import gradio as gr
from pydantic import BaseModel


####
openai = OpenAI(
    api_key=os.getenv('OPENAI_API_KEY'),
    base_url=os.getenv("OPENAI_API_BASE"),
)

####
reader = PdfReader(Path("docs") / "me" / "linkedin.pdf")
linkedin = ""

for page in reader.pages:
    text = page.extract_text()
    if text:
        linkedin += text

#print(f"==> linkedin:\n{linkedin}")

with open(Path("docs") / "me" / "summary.txt", "r", encoding="utf-8") as f:
    summary = f.read()

name = "Ed Donner"

system_prompt = f"""
You are acting as {name}. You are answering questions on {name}'s website, particularly questions 
related to {name}'s career, background, skills and experience. Your responsibility is to represent 
{name} for interactions on the website as faithfully as possible. You are given a summary of 
{name}'s background and LinkedIn profile which you can use to answer questions. Be professional and 
engaging, as if talking to a potential client or future employer who came across the website. If 
you don't know the answer, say so.
""".replace("\n", "").strip()

system_prompt += f"""

## Summary:
{summary}

## LinkedIn Profile:
{linkedin}

With this context, please chat with the user, always staying in character as {name}.
"""

#print(f"\n==> system_prompt:\n{system_prompt}")

with open(Path("data") / "lab3_system_prompt.txt", 'w', encoding="utf-8") as f:
    f.write(system_prompt)

####
def chat_v1(message, history):
    messages = [{"role": "system", "content": system_prompt}] + history + [{"role": "user", "content": message}]
    response = openai.chat.completions.create(model="gpt-4o-mini", messages=messages)
    return response.choices[0].message.content


gr.ChatInterface(chat_v1, type="messages").launch()
