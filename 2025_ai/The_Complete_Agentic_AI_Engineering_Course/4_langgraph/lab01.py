#!/usr/bin/env python3
import os, random
from pathlib import Path
from typing import Annotated

from dotenv import load_dotenv
from pydantic import BaseModel
from langchain_openai import ChatOpenAI
from langgraph.graph import StateGraph, START, END
from langgraph.graph.message import add_messages
import gradio as gr


####
load_dotenv(Path("configs") / "local.env", override=True)
basename = Path(os.sys.argv[0]).name
print(f"--> basename: {basename}")
Path("data").mkdir(parents=True, exist_ok=True)


####
class State(BaseModel):
    messages: Annotated[list, add_messages]

graph_builder = StateGraph(State)

####
llm = ChatOpenAI(model="gpt-4o-mini")
llm_with_tools = llm.bind_tools(tools)


def chatbot(state: State):
    return {"messages": [llm_with_tools.invoke(state["messages"])]}

graph_builder.add_node("chatbot", chatbot)
graph_builder.add_node("tools", ToolNode(tools=tools))

graph_builder.add_conditional_edges( "chatbot", tools_condition, "tools")

# Any time a tool is called, we return to the chatbot to decide the next step
graph_builder.add_edge("tools", "chatbot")
graph_builder.add_edge(START, "chatbot")

graph = graph_builder.compile()
display(Image(graph.get_graph().draw_mermaid_png()))

def chat(user_input: str, history):
    result = graph.invoke({"messages": [{"role": "user", "content": user_input}]})
    return result["messages"][-1].content


with open(Path("data") / f"{basename}.graph.png", 'wb') as f:
    f.write(graph.get_graph().draw_mermaid_png())

gr.ChatInterface(chat, type="messages").launch()
