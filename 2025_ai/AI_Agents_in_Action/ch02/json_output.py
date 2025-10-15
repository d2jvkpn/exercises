import os
from pathlib import Path

from dotenv import load_dotenv
from openai import OpenAI

####
load_dotenv(Path("configs") / "local.env")
base_url = os.getenv('OPENAI_BASE_URL')
api_key = os.getenv('OPENAI_API_KEY', "https://api.openai.com/v1")

####
client = OpenAI(api_key=api_key, base_url=base_url)

# Example function to query ChatGPT
def ask_chatgpt(messages, temperature=0.7):
    response = client.chat.completions.create(
        #model="gpt-4-1106-preview",
        model="gpt-4-mini",
        messages=messages,
        temperature=temperature,
        response_format={"type": "json_object"},
    )

    return response

####
messages = [
    {
        "role": "system",
        "content": "You are a helpful assistant and always output JSON.",
    },
    {
        "role": "user",
        "content": "What is the captial of France?",
    },
    {
        "role": "assistant",
        "content": "The capital of France is Paris.",
    },
    {
        "role": "user",
        "content": "What is an interesting fact of Paris.",
    },
]

response = ask_chatgpt(messages)

print(response.choices[0].message.content)
