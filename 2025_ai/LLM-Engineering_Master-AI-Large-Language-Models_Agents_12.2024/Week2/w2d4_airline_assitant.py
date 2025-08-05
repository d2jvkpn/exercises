#!/usr/bin/env python3
import os, json
from pathlib import Path

from dotenv import load_dotenv
from openai import OpenAI
import gradio as gr


load_dotenv(Path("configs") / "local.env")

client = OpenAI(
    api_key=os.environ.get("api_key"),
    base_url=os.environ.get("api_base"),
)
model = os.environ["model"]


system_message = """
You are a helpful assistant for an Airline called FlightAI. Give short, courteous answers, no more 
than 1 sentence. Always be accurate. If you don't know the answer, say so.
"""

def get_ticket_price(destination_city):
    #print(f"<-- get_ticket_price: {destination_city}")
    ticket_prices = {
        "london": "$799",
        "paris": "$899",
        "tokyo": "$1400",
        "berlin": "$499",
    }

    city = destination_city.lower()

    return ticket_prices.get(city, "Unknown")


# There's a particular dictionary structure that's required to describe our function:
price_function = {
    "name": "get_ticket_price",

    "description": "Get the price of a return ticket to the destination city. Call this " + \
        "whenever you need to know the ticket price, for example when a customer asks " + \
        "'How much is a ticket to this city'",

    "parameters": {
        "type": "object",
        "properties": {
            "destination_city": {
                "type": "string",
                "description": "The city that the customer wants to travel to",
            },
        },
        "required": ["destination_city"],
        "additionalProperties": False,
    }
}


def handle_tool_call(message):
    # print(f"<-- handle_tool_call: {message}")
    # ChatCompletionMessage(content=None, refusal=None, role='assistant', annotations=[], audio=None, function_call=None, tool_calls=[ChatCompletionMessageToolCall(id='call_jHoMtcTadd4oBcIf06Mr3nOv', function=Function(arguments='{"destination_city":"London"}', name='get_ticket_price'), type='function')])
    tool_call = message.tool_calls[0]
    function = tool_call.function
    # communicate in json: json.loads and json.dumps
    arguments = json.loads(function.arguments)

    print(f"~~~ tool_call: type={tool_call.type}, function={function.name}, arguments={function.arguments}")

    city = arguments.get('destination_city')
    price = get_ticket_price(city)

    response = {
        "role": "tool",
        "content": json.dumps({"destination_city": city, "price": price}),
        "tool_call_id": tool_call.id,
    }

    return response, city


def chat(message, history):
    messages = [{"role": "system", "content": system_message.strip()}]
    messages.extend(history)
    messages += [{"role": "user", "content": message}]

    #response = openai.chat.completions.create(
    response = client.chat.completions.create(
        model=model, messages=messages,
        tools=[{"type": "function", "function": price_function}],
        stream=False,
    )

    if response.choices[0].finish_reason == "tool_calls":
        msg = response.choices[0].message
        response, city = handle_tool_call(msg)

        if response:
            messages.extend([msg, response])
            response = client.chat.completions.create(model=model, messages=messages)

    return response.choices[0].message.content


view = gr.ChatInterface(fn=chat, type="messages", title="Airline Assitant")
view.launch(
    share=False,
    server_name="127.0.0.1",
    server_port=7860,
)
