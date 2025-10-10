#!/usr/bin/env python3
import os, asyncio
from pathlib import Path
from dotenv import load_dotenv
load_dotenv(Path("configs") / "local.env", override=True)

from openai import AsyncOpenAI
from agents import Agent, Runner, trace
from agents import set_default_openai_client, set_tracing_disabled # set_default_openai_api

####
openai = AsyncOpenAI(
    base_url=os.getenv("OPENAI_BASE_URL", "https://api.openai.com/v1"),
    api_key=os.getenv("OPENAI_API_KEY"),
)

set_tracing_disabled(True)
set_default_openai_client(openai)
#set_default_openai_api("chat_completions")

####
agent = Agent(
    name="Jokester",
    instructions="You are a joke teller", # system prompt
    model="gpt-4o-mini",
)

# Run the joke with Runner.run(agent, prompt) then print final_output
# https://platform.openai.com/traces
async def run():
    with trace("Telling a joke"):
        result = await Runner.run(
            agent,
            "Tell a joke about Autonomous AI Agents", # user prompt
        )

        # print(result.final_output)
        return result

####
result = asyncio.run(run())
print(f"==> Result: {result}")
