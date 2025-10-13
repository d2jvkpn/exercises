#!/usr/bin/env python3
import os, json
from pathlib import Path
from dotenv import load_dotenv
load_dotenv(Path("configs")/"local.env", override=True)

from openai import OpenAI
import requests
from pypdf import PdfReader
import gradio as gr


####
system_prompt_template = """
You are acting as {name}. You are answering questions on {name}'s website, particularly questions 
related to {name}'s career, background, skills and experience. Your responsibility is to represent 
{name} for interactions on the website as faithfully as possible. You are given a summary of 
{name}'s background and LinkedIn profile which you can use to answer questions. Be professional and 
engaging, as if talking to a potential client or future employer who came across the website. If 
you don't know the answer to any question, use your record_unknown_question tool to record the 
question that you couldn't answer, even if it's about something trivial or unrelated to career. If 
the user is engaging in discussion, try to steer them towards getting in touch via email; ask for 
their email and record it using your record_user_details tool.

## Summary:
{summary}

## LinkedIn Profile:
{linkedin}

With this context, please chat with the user, always staying in character as {name}.
""".replace(" \n", " ").strip()

pushover_url = "https://api.pushover.net/1/messages.json"
pushover_user_key = os.getenv("PUSHOVER_USER")
#pushover_app = os.getenv("PUSHOVER_APP")
pushover_token = os.getenv("PUSHOVER_TOKEN")
pushover_device = os.getenv("PUSHOVER_DEVICE")

####
openai = OpenAI(
    api_key=os.getenv('OPENAI_API_KEY'),
    base_url=os.getenv("OPENAI_BASE_URL"),
)

#####
name = "Ed Donner"

reader = PdfReader(Path("docs") / "me" / "linkedin.pdf")
linkedin = ""

for page in reader.pages:
    text = page.extract_text()
    if text:
        linkedin += text

with open(Path("docs") / "me" / "summary.txt", "r", encoding="utf-8") as f:
    summary = f.read()


####
def push(message):
    print(f"--> Push: {message}")

    payload = {
        "user": pushover_user_key,
        "token": pushover_app_token,
        "device": pushover_device,
        "message": message,
    }

    requests.post(pushover_url, data=payload)

#push("HEY!!")

def record_user_details(email, name="Name not provided", notes="not provided"):
    push(f"Recording interest from {name} with email {email} and notes {notes}")
    return {"recorded": "ok"}

def record_unknown_question(question):
    push(f"Recording {question} asked that I couldn't answer")
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
                "description": "The email address of this user"
            },
            "name": {
                "type": "string",
                "description": "The user's name, if they provided it"
            }
            ,
            "notes": {
                "type": "string",
                "description": "Any additional information about the conversation that's worth recording to give context"
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
                "description": "The question that couldn't be answered"
            },
        },
        "required": ["question"],
        "additionalProperties": False,
    }
}

tools = [
    { "type": "function", "function": record_user_details_json },
    { "type": "function", "function": record_unknown_question_json },
]


####
def handle_tool_calls(tool_calls):
    results = []
    for tool_call in tool_calls:
        tool_name = tool_call.function.name
        arguments = json.loads(tool_call.function.arguments)
        print(f"--> Tool called: {tool_name}: {arguments}", flush=True)

        # THE BIG IF STATEMENT!!!
        if tool_name == "record_user_details":
            result = record_user_details(**arguments)
        elif tool_name == "record_unknown_question":
            result = record_unknown_question(**arguments)

        results.append({
            "role": "tool",
            "content": json.dumps(result),
            "tool_call_id": tool_call.id,
        })

    return results

system_prompt = system_prompt_template.format(name=name, summary=summary, linkedin=linkedin)

def chat(message, history):
    messages = [{"role": "system", "content": system_prompt}] + history
    messages.append({"role": "user", "content": message})

    done = False
    while not done:
        response = openai.chat.completions.create(
            model="gpt-4o-mini", messages=messages, tools=tools,
        )

        finish_reason = response.choices[0].finish_reason

        # If the LLM wants to call a tool, we do that!
        if finish_reason == "tool_calls":
            message = response.choices[0].message
            results = handle_tool_calls(message.tool_calls)
            messages.append(message)
            messages.extend(results)
        else:
            done = True

    return response.choices[0].message.content

gr.ChatInterface(chat, type="messages").launch()
