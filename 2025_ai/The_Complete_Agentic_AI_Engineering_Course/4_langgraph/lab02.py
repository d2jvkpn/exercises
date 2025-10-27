#!/usr/bin/env python3
import os, random, sqlite3
from pathlib import Path
from typing import Annotated, TypedDict

from dotenv import load_dotenv
import requests
from pydantic import BaseModel
from langchain_openai import ChatOpenAI
from langchain.agents import Tool
from langgraph.graph import StateGraph, START, END
from langgraph.graph.message import add_messages
from langgraph.prebuilt import ToolNode, tools_condition
from langchain_community.utilities import GoogleSerperAPIWrapper
from langgraph.checkpoint.memory import MemorySaver
from langgraph.checkpoint.sqlite import SqliteSaver
import gradio as gr


####
load_dotenv(Path("configs") / "local.env", override=True)
basename = Path(os.sys.argv[0]).name
print(f"--> basename: {basename}")
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

tools = [tool_search, tool_push]

#memory_saver = MemorySaver()
conn = sqlite3.connect(Path("data") / "memory.db", check_same_thread=False)
sql_memory = SqliteSaver(conn)

####
class State(TypedDict):
    messages: Annotated[list, add_messages]

llm = ChatOpenAI(model="gpt-4o-mini")
llm = llm.bind_tools(tools)

####
def chatbot(state: State):
    return {"messages": [llm.invoke(state["messages"])]}

graph_builder = StateGraph(State)
graph_builder.add_node("chatbot", chatbot)
graph_builder.add_node("tools", ToolNode(tools=tools))

graph_builder.add_conditional_edges( "chatbot", tools_condition, "tools")
# Any time a tool is called, we return to the chatbot to decide the next step
graph_builder.add_edge("tools", "chatbot")

graph_builder.add_edge(START, "chatbot")
graph_builder.add_edge("chatbot", END)

#graph = graph_builder.compile(checkpointer=memory_saver)
graph = graph_builder.compile(checkpointer=sql_memory)

####
with open(Path("data") / f"{basename}.graph.png", 'wb') as f:
    f.write(graph.get_graph().draw_mermaid_png())

def chat(user_input: str, history):
    result = graph.invoke(
        {"messages": [
            {"role": "user", "content": user_input},
        ]},
        config={"configurable": {"thread_id": "1"}},
    )

    return result["messages"][-1].content

gr.ChatInterface(chat, type="messages").launch()
#graph.get_state(config)
#list(graph.get_state_history(config))
