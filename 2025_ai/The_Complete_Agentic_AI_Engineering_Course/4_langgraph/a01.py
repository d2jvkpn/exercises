#!/usr/bin/env python3
import sqlite3
from pathlib import Path
from typing import TypedDict, Annotated, List

from dotenv import load_dotenv
from langgraph.graph import StateGraph, END
from langgraph.checkpoint.sqlite import SqliteSaver
from langchain_openai import ChatOpenAI


load_dotenv(Path("configs") / "local.env")
app_name = "demo-001"

# 1) 定义状态
class State(TypedDict):
    question: str
    thoughts: Annotated[List[str], "append"]
    answer: str

llm = ChatOpenAI(model="gpt-4o-mini")                 # 换成你本地/私有模型也行
conn = sqlite3.connect(Path("data") / f"{app_name}.memory.db", check_same_thread=False)
memory = SqliteSaver(conn)  # Checkpointer

# 2) 定义节点
def plan(state: State):
    plan_text = f"分解：回答 `{state['question']}` 的关键点是……"
    return {"thoughts": [plan_text]}

def answer(state: State):
    prompt = f"问题：{state['question']}\n要点：{'\n'.join(state['thoughts'])}"
    resp = llm.invoke(prompt)
    return {"answer": resp.content}

# 3) 组图
g = StateGraph(State)
g.add_node("plan", plan)
g.add_node("answer", answer)
g.set_entry_point("plan")
g.add_edge("plan", "answer")
g.add_edge("answer", END)

app = g.compile(checkpointer=memory)

# 4) 运行（带可恢复的线程ID）
config = {"thread_id": app_name}

for event in app.stream({"question": "LangGraph 的优势是什么？"}, {"configurable": config}):
    print(event)
# 之后随时可用同一个 thread_id 继续/回放
