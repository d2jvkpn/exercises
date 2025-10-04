#!/usr/bin/env python3
import os, json
from pathlib import Path
from dotenv import load_dotenv
_ = load_dotenv(Path("configs") / "local.env", override=True)

import requests
from openai import OpenAI
from pypdf import PdfReader
import gradio as gr


####
system_prompt_tempate = """
You are acting as {name}. You are answering questions on {name}'s website, 
particularly questions related to {name}'s career, background, skills and experience. Your 
responsibility is to represent {name} for interactions on the website as faithfully as possible. 
You are given a summary of {name}'s background and LinkedIn profile which you can use to answer 
questions. Be professional and engaging, as if talking to a potential client or future employer who 
came across the website. If you don't know the answer to any question, use your 
record_unknown_question tool to record the question that you couldn't answer, even if it's about 
something trivial or unrelated to career. If the user is engaging in discussion, try to steer them 
towards getting in touch via email; ask for their email and record it using your 
record_user_details tool.

## Summary:
{summary}

## LinkedIn Profile:
{linkedin}

With this context, please chat with the user, always staying in character as {name}.
""".replace(" \n", " ").strip()


####
def push(text):
    print(f"--> Push: {text}")

    requests.post(
        "https://api.pushover.net/1/messages.json",
        data={
            "token": os.getenv("PUSHOVER_TOKEN"),
            "user": os.getenv("PUSHOVER_USER"),
            "device": os.getenv("PUSHOVER_DEVICE"),
            "message": text,
        }
    )

def record_user_details(email, name="Name not provided", notes="not provided"):
    push(f"Recording {name} with email {email} and notes {notes}")
    return {"recorded": "ok"}

def record_unknown_question(question):
    push(f"Recording {question}")
    return {"recorded": "ok"}

####
record_user_details_json = {
    "name": "record_user_details",
    "description": "Use this tool to record that a user is interested in being in touch and provided an email address",
    "parameters": {
        "type": "object",
        "properties": {
            "email": {
                "type": "string",
                "description": "The email address of this user",
            },
            "name": {
                "type": "string",
                "description": "The user's name, if they provided it",
            },
            "notes": {
                "type": "string",
                "description": "Any additional information about the conversation that's worth recording to give context",
            }
        },
        "required": ["email"],
        "additionalProperties": False,
    }
}

record_unknown_question_json = {
    "name": "record_unknown_question",
    "description": "Always use this tool to record any question that couldn't be answered as you didn't know the answer",
    "parameters": {
        "type": "object",
        "properties": {
            "question": {
                "type": "string",
                "description": "The question that couldn't be answered",
            },
        },
        "required": ["question"],
        "additionalProperties": False,
    }
}


####
class Me:
    def __init__(self):
        self.openai = OpenAI(
            api_key=os.getenv('OPENAI_API_KEY'),
            base_url=os.getenv("OPENAI_API_BASE"),
        )

        self.name = "Ed Donner"
        self.linkedin = ""

        for page in PdfReader(Path("docs") / "me" / "linkedin.pdf").pages:
            text = page.extract_text()
            if text:
                self.linkedin += text

        with open(Path("docs") / "me" / "summary.txt", "r", encoding="utf-8") as f:
            self.summary = f.read()

        self.tools = [
            {"type": "function", "function": record_user_details_json},
            {"type": "function", "function": record_unknown_question_json},
        ]

    def handle_tool_call(self, tool_calls):
        results = []

        for tool_call in tool_calls:
            tool_name = tool_call.function.name
            arguments = json.loads(tool_call.function.arguments)

            print(f"--> Tool called: {tool_name}, {arguments}", flush=True)
            tool = globals().get(tool_name)
            result = tool(**arguments) if tool else {}

            results.append({
                "role": "tool",
                "content": json.dumps(result),
                "tool_call_id": tool_call.id,
            })

        return results

    def chat(self, message, history):
        system_prompt = system_prompt_tempate.format(
            name=self.name, summary=self.summary, linkedin=self.linkedin,
        )

        messages = [{"role": "system", "content": system_prompt}] + history + [{"role": "user", "content": message}]

        done = False
        while not done:
            response = self.openai.chat.completions.create(
                model="gpt-4o-mini", messages=messages, tools=self.tools,
            )

            if response.choices[0].finish_reason=="tool_calls":
                message = response.choices[0].message
                tool_calls = message.tool_calls
                results = self.handle_tool_call(tool_calls)
                messages.append(message)
                messages.extend(results)
            else:
                done = True

        return response.choices[0].message.content


if __name__ == "__main__":
    me = Me()
    gr.ChatInterface(me.chat, type="messages").launch()
