#!/usr/bin/env python3
import os, json, base64, subprocess, io
from pathlib import Path
from datetime import datetime, UTC

import uuid
from dotenv import load_dotenv
from openai import OpenAI
import gradio as gr
from PIL import Image

#from pydub import AudioSegment
#from pydub.playback import play
#import simpleaudio as sa

# $ apt-get install libasound2-dev
# $ pip install pydub simpleaudio arithmetic

#### 1. client
load_dotenv(Path("configs") / "local.env")

client = OpenAI(
    api_key=os.environ.get("api_key"),
    base_url=os.environ.get("api_base"),
)

system_message = """
You are a helpful assistant for an Airline called FlightAI. Give short, courteous answers, no more 
than 1 sentence. Always be accurate. If you don't know the answer, say so.
"""

def ts_uuid_name():
    now = datetime.now(UTC)
    base = "{}_{}".format(int(now.timestamp()), uuid.uuid4())
    return now.strftime("%F-utc"), base

#### 2. assistant
def get_ticket_price(destination_city):
    #print(f"<-- get_ticket_price: {destination_city}")
    ticket_prices = {
        "london": "$799",
        "paris": "$899",
        "tokyo": "$1400",
        "berlin": "$499",
    }

    city = destination_city.lower()

    return ticket_prices.get(city, "unknown")


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
    # ChatCompletionMessage(content=None, refusal=None, role='assistant', annotations=[], audio=None,
    # function_call=None, tool_calls=[ChatCompletionMessageToolCall(id='call_jHoMtcTadd4oBcIf06Mr3nOv',
    # function=Function(arguments='{"destination_city":"London"}', name='get_ticket_price'), type='function')])
    tool_call = message.tool_calls[0]
    function = tool_call.function
    # communicate in json: json.loads and json.dumps
    arguments = json.loads(function.arguments)

    # print(f"~~~ tool_call: type={tool_call.type}, function={function.name}, arguments={function.arguments}")

    city = arguments.get('destination_city')
    price = get_ticket_price(city)

    response = {
        "role": "tool",
        "content": json.dumps({"destination_city": city, "price": price}),
        "tool_call_id": tool_call.id,
    }

    return response, city


#### 3. artist
def artist(city):
    response = client.images.generate(
        model="dall-e-3",
        prompt=f"An image representing a vacation in {city}, showing tourist spots and " + \
            "everything unique about {city}, in a vibrant pop-art style.",
        size="1024x1024",
        n=1,
        response_format="b64_json",
    )

    image_b64 = response.data[0].b64_json
    image_data = base64.b64decode(image_b64)

    return Image.open(io.BytesIO(image_data))
    #image = artist("New York City")
    #display(image)


#### 4. talker
def talker_ffmpeg(message):
    message = message.strip()

    response = client.audio.speech.create(
        model="tts-1",
        voice="onyx", # alloy, onyx
        input=message,
    )

    directory, base = ts_uuid_name()
    directory = Path("data") / "audio" / directory
    directory.mkdir(parents=True, exist_ok=True)
    prefix = directory / base
    print(f"--> saving audio prefix: {prefix}")

    with open(prefix.with_suffix(".mp3"), "wb") as f:
        f.write(response.content)

    with open(prefix.with_suffix(".txt"), "w", encoding="utf-8") as f:
        f.write(message+"\n")

    subprocess.call(
        ["ffplay", "-nodisp", "-autoexit", "-hide_banner", prefix.with_suffix(".mp3")],
        stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL,
    )


#def talker_pydub(message):
#    response = client.audio.speech.create(
#        model="tts-1",
#        voice="onyx", # alloy, onyx
#        input=message,
#    )

#    audio = AudioSegment.from_file(io.BytesIO(response.content), format="mp3")

#    try:
#        play(audio)
#    except Exception as e:
#        print(f"~~~ talker_pydub error: {e}")
#    finally:
#        print("~~~ talker_pydub exit")


#### 5. chat
def chat_v1(history):
    image = None
    messages = [{"role": "system", "content": system_message}] + history

    response = client.chat.completions.create(
        model="gpt-4o", messages=messages,
        tools=[{"type": "function", "function": price_function}],
        stream=False,
    )

    if response.choices[0].finish_reason == "tool_calls":
        msg = response.choices[0].message
        # print(f"~~~ message: {msg}")
        # ChatCompletionMessage(content=None, refusal=None, role='assistant', annotations=[],
        # audio=None, function_call=None, tool_calls=[ChatCompletionMessageToolCall(
        #   id='call_YKA81w9Ky4m1YKSUpQB7fbWN',
        #   function=Function(arguments='{"destination_city":"London"}',
        #   name='get_ticket_price'), type='function')])

        response, city = handle_tool_call(msg)
        # print("???", [msg, response])
        messages.extend([msg, response])
        #image = artist(city)
        response = client.chat.completions.create(model="gpt-4o", messages=messages)

    reply = response.choices[0].message.content
    history.append({"role":"assistant", "content": reply})
    yield history, image

    # Comment out or delete the next line if you'd rather skip Audio for now..
    talker_ffmpeg(reply)

    #return history, image


def chat_v2(history):
    image = None
    messages = [{"role": "system", "content": system_message}] + history

    response = client.chat.completions.create(
        model="gpt-4o", messages=messages,
        tools=[{"type": "function", "function": price_function}],
        stream=True,
    )

    history.append({"role":"assistant", "content": ""})

    tool_calls = []
    for chunk in response:
        choice = chunk.choices[0]

        if hasattr(choice.delta, 'tool_calls') and choice.delta.tool_calls:
            for tool_call in choice.delta.tool_calls:
                # print("~~~ tool_call:", tool_call)
                if tool_call.index == 0 and tool_call.index + 1 > len(tool_calls):
                    tool_calls.append({
                        "id": tool_call.id,
                        "type": tool_call.type,
                        "function": {"name": tool_call.function.name, "arguments": ""},
                     })

                if tool_call.function.arguments:
                    tool_calls[-1]["function"]["arguments"] += tool_call.function.arguments

        if hasattr(choice.delta, 'content') and choice.delta.content:
            # print(f"~~ chunk.delta.content: {choice.delta.content}")
            history[-1]['content'] += choice.delta.content
            yield history, image

    if history[-1]['content']:
        talker_ffmpeg(history[-1]['content'])

    # print("~~~ tool_calls:", tool_calls)
    if len(tool_calls) > 0 and tool_calls[0]["function"]["name"] == "get_ticket_price":
        tool_call = tool_calls[0]
        args = json.loads(tool_call['function']['arguments'])
        city = args.get('destination_city')
        price = get_ticket_price(**args)

        msg = {
            "role": "assistant",
            "content": None,
            "tool_calls": [tool_call],
        }

        response = {
            "role": "tool",
            "content": json.dumps({"destination_city": city, "price": price}),
            "tool_call_id": tool_call['id'],
        }

        messages.extend([msg, response])
        #image = artist(city)
        response = client.chat.completions.create(model="gpt-4o", messages=messages, stream=True)
        #msg = response.choices[0].message
        history.append({"role":"assistant", "content": ""})
        for chunk in response:
            history[-1]['content'] += chunk.choices[0].delta.content or ""
            yield history, image

        talker_ffmpeg(history[-1]['content'])



#### 6. webui
with gr.Blocks() as ui:
    with gr.Row():
        chatbot = gr.Chatbot(height=500, type="messages")
        image_output = gr.Image(height=500)
    with gr.Row():
        entry = gr.Textbox(label="Chat with our AI Assistant:")

    def do_entry(message, history):
        history += [{"role":"user", "content":message}]
        return "", history

    entry.submit(do_entry, inputs=[entry, chatbot], outputs=[entry, chatbot]).then(
        chat_v2, inputs=chatbot, outputs=[chatbot, image_output],
    )


ui.launch(inbrowser=False)
