#!/usr/bin/env python3
import os
from pathlib import Path
from datetime import datetime, UTC

from dotenv import load_dotenv
import openai, uuid


load_dotenv(dotenv_path="configs/local.env")

client = openai.OpenAI(
    api_key=os.environ.get("api_key"),
    base_url=os.environ.get("api_base"),
)

def ts_uuid_name():
    now = datetime.now(UTC)
    base = "{}_{}".format(int(now.timestamp()), uuid.uuid4())
    return now.strftime("%F-utc"), base


message = "Hello, world"

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


with open(prefix.with_suffix(".mp3"), "rb") as audio_file:
    transcription = client.audio.transcriptions.create(
        model="whisper-1", file=audio_file, response_format="text",
    )

    print(f"transcription: {transcription}")
