#!/usr/bin/env python3
import os, random, sqlite3
from pathlib import Path
from typing import Annotated, TypedDict

from dotenv import load_dotenv
import requests
from pydantic import BaseModel
import gradio as gr
import nest_asyncio
nest_asyncio.apply()
from langchain_openai import ChatOpenAI
from langchain_core.tools import Tool
from langgraph.graph import StateGraph, START, END
from langgraph.graph.message import add_messages
from langgraph.prebuilt import ToolNode, tools_condition
from langchain_community.utilities import GoogleSerperAPIWrapper
from langgraph.checkpoint.memory import MemorySaver
from langgraph.checkpoint.sqlite import SqliteSaver
from langchain_community.agent_toolkits import PlayWrightBrowserToolkit
from langchain_community.tools.playwright.utils import create_async_playwright_browser
# If you get a NotImplementedError here or later, see the Heads Up at the top of the notebook


####
load_dotenv(Path("configs") / "local.env", override=True)
basename = Path(os.sys.argv[0]).name
#print(f"--> basename: {basename}")
Path("data").mkdir(parents=True, exist_ok=True)

####
serper = GoogleSerperAPIWrapper()
#serper.run("What is the capital of France?")

tool_search =Tool(
    name="search",
    func=serper.run,
    description="Useful for when you need more information from an online search",
)
#tool_search.invoke("What is the capital of France?")

pushover_data = {"token": os.getenv("PUSHOVER_TOKEN"), "user": os.getenv("PUSHOVER_USER")}
def push(text: str):
    """Send a push notification to the user"""
    pushover_data["message"] = text

    requests.post(
        "https://api.pushover.net/1/messages.json",
        data=pushover_data,
    )

tool_push = Tool(
    name="send_push_notification",
    func=push,
    description="useful for when you want to send a push notification",
)
#tool_push.invoke("Hello, me!")

async_browser =  create_async_playwright_browser(headless=False)  # headful mode
pwb_toolkit = PlayWrightBrowserToolkit.from_browser(async_browser=async_browser)
#import textwrap
#navigate_tool = tool_dict.get("navigate_browser")
#extract_text_tool = tool_dict.get("extract_text")

#await navigate_tool.arun({"url": "https://www.cnn.com"})
#text = await extract_text_tool.arun({})
#print(textwrap.fill(text))

tools = pwb_toolkit.get_tools() + [tool_search, tool_push]

print("tools:")
for tool in tools:
    print(f"- name: {tool.name}\n  description: {repr(tool.description)}\n  args: {tool.args}")

####
#memory_saver = MemorySaver()
conn = sqlite3.connect(Path("data") / "lab02.memory.db", check_same_thread=False)
sql_memory = SqliteSaver(conn)

class State(TypedDict):
    messages: Annotated[list, add_messages]

llm = ChatOpenAI(model="gpt-4o-mini")
llm = llm.bind_tools(tools)

####
def chatbot(state: State):
    resp = llm.invoke(state["messages"])
    return {"messages": [resp]}

graph = StateGraph(State)
graph.add_node("chatbot", chatbot)
graph.add_node("tools", ToolNode(tools=tools))

graph.add_conditional_edges("chatbot", tools_condition, "tools")
# Any time a tool is called, we return to the chatbot to decide the next step
graph.add_edge("tools", "chatbot")

graph.add_edge(START, "chatbot")
#graph.set_entry_point("chatbot")
graph.add_edge("chatbot", END)

#graph = graph.compile(checkpointer=memory_saver)
graph = graph.compile(checkpointer=sql_memory)

####
with open(Path("data") / f"{basename}.graph.png", 'wb') as f:
    f.write(graph.get_graph().draw_mermaid_png())

config = {"configurable": {"thread_id": "1"}}

def chat(user_input: str, history):
    msgs = [{ "role": "user", "content": user_input }]
    result = graph.invoke({"messages": msgs}, config=config)

    return result["messages"][-1].content

gr.ChatInterface(chat, type="messages").launch()
# user: Send me a push notification with a news headline from CNN in Chinese.
# user: Please send me a push notification with the current USD/GBP exchange rate.
# user: 发一个当前 USD/RMB 汇率的推送通知

os.sys.exit(0)
state = graph.get_state(config)
for msg in state.values['messages']:
    print(f"==> {type(msg)}\n{msg}\n")

for v in graph.get_state_history(config):
    print(f"==> state:\n{v}\n")
