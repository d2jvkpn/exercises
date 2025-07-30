#!/usr/bin/env python3
import glob


context = {}

####
employees = glob.glob("data/knowledge-base/employees/*")

for employee in employees:
    name = employee.split(' ')[-1][:-3]
    with open(employee, "r", encoding="utf-8") as f:
        doc = f.read()

    context[name] = doc

####
products = glob.glob("data/knowledge-base/products/*")

for product in products:
    name = product.split(os.sep)[-1][:-3]
    doc = ""
    with open(product, "r", encoding="utf-8") as f:
        doc = f.read()
    context[name]=doc


system_prompt = """
You are an expert in answering accurate questions about Insurellm, the Insurance Tech company. Give
brief, accurate answers. If you don't know the answer, say so. Do not make anything up if you
haven't been provided with relevant context.
""".strip().replace("\n", " ")


def get_relevant_context(message):
    relevant_context = []

    for title, content in context.items():
        if title.lower() in message.lower():
            relevant_context.append(content)

    return relevant_context

get_relevant_context("Who is lancaster?")


def add_context(message):
    relevant_context = get_relevant_context(message)

    if relevant_context:
        message += "\n\nThe following additional context might be relevant in answering this question:\n\n"
        for relevant in relevant_context:
            message += relevant + "\n\n"

    return message


def chat(message, history):
    messages = [{"role": "system", "content": system_prompt}] + history

    msg = add_context(message)
    messages.append({"role": "user", "content": msg})

    response = openai.chat.completions.create(model="gpt-4o-mini", messages=messages, stream=True)

    response = ""
    for chunk in response:
        response += chunk.choices[0].delta.content or ''
        yield response


view = gr.ChatInterface(chat, type="messages").launch()
