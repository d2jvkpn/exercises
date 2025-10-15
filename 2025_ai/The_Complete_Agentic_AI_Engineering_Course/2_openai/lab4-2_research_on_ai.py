#!/usr/bin/env python3
import os, json, asyncio
from typing import Dict
from pathlib import Path
from dotenv import load_dotenv
load_dotenv(Path("configs") / "local.env", override=True)

from openai import AsyncOpenAI
from openai.types.responses import ResponseTextDeltaEvent
from agents import set_default_openai_client, set_tracing_disabled       # set_default_openai_api
from agents import Agent, Runner, WebSearchTool, GuardrailFunctionOutput # OpenAIChatCompletionsModel
from agents import function_tool, trace, input_guardrail                 # gen_trace_id
from agents.model_settings import ModelSettings
#import sendgrid
#from sendgrid.helpers.mail import Mail, Email, To, Content
from pydantic import BaseModel, Field

#### 1. init
llm_client = AsyncOpenAI(
    base_url=os.getenv("OPENAI_BASE_URL", "https://api.openai.com/v1"),
    api_key=os.getenv("OPENAI_API_KEY"),
)

model = "gpt-4o-mini"

set_tracing_disabled(True)
set_default_openai_client(llm_client)
#set_default_openai_api("chat_completions")

#### 2. search agent
# role, goal(inputs and outputs) and rules
instructions = "你是一名研究助理。给定一个搜索词后，你需要在网上搜索该词条，并撰写一份简明的结果摘要。摘要应为\
2-3 段，总字数不超过 300 个英文单词。需抓住要点，语言简洁，不必使用完整句子或完美语法。你的摘要将被他人用于综合\
报 告，因此务必抓住核心信息，去除无关内容。除摘要本身外，不要添加任何额外评论。"

search_agent = Agent(
    name="SearchAgent",
    instructions=instructions,
    model=model,
    model_settings=ModelSettings(tool_choice="required"),
    tools=[
        WebSearchTool(search_context_size="low"),
    ],
)

async def test_search_agent(
    query = "2025 年最新的 AI 智能体（AI Agent）框架",
    save=Path("data") / "lab4-2_search_agent.json",
):
    with trace("Search"):
        result = await Runner.run(search_agent, query)

    data = {
        "name": "search_agent",
        "query": query,
        "result": result.final_output,
    }

    with open(save, 'w') as f:
        #f.write(result.final_output)
        json.dump(data, f, indent=2, ensure_ascii=False)

#asyncio.run(test_search_agent())

#### 3. planner agent
HOW_MANY_SEARCHES = 3

instructions = f"你是一名乐于助人的研究助理。给定一个问题后，请构思一组网络搜索关键词，以便最有效地回答该问题。\
输出 {HOW_MANY_SEARCHES} 个需要查询的搜索词。"

# Use Pydantic to define the Schema of our response - this is known as "Structured Outputs"
# With massive thanks to student Wes C. for discovering and fixing a nasty bug with this!

class WebSearchItem(BaseModel):
    reason: str = Field(description="Your reasoning for why this search is important to the query.")
    query: str = Field(description="The search term to use for the web search.")

class WebSearchPlan(BaseModel):
    searches: list[WebSearchItem] = Field(
        description="A list of web searches to perform to best answer the query.",
    )

planner_agent = Agent(
    name="PlannerAgent",
    instructions=instructions,
    model=model,
    output_type=WebSearchPlan,
)

async def test_planner_agent(
    query = "2025 年最新的 AI 智能体（AI Agent）框架",
    save=Path("data") / "lab4-2_planner_agent.json",
):
    with trace("Planner"):
        result = await Runner.run(planner_agent, query)

    searches = [{"reason": v.reason, "query": v.query} for v in result.final_output.searches]
    data = {
      "name": "planner_agent",
      "query": query,
      "result": searches,
    }

    with open(save, 'w') as f:
        json.dump(data, f, indent=2, ensure_ascii=False)

#asyncio.run(test_planner_agent())

#### 4. email agent
@function_tool
def send_text_email(body: str):
    """Send out an email with the given body to all sales prospects"""
    #sg = sendgrid.SendGridAPIClient(api_key=os.environ.get('SENDGRID_API_KEY'))
    #from_email = Email("ed@edwarddonner.com")  # Change to your verified sender
    #to_email = To("ed.donner@gmail.com")  # Change to your recipient
    #content = Content("text/plain", body)
    #mail = Mail(from_email, to_email, "Sales email", content).get()
    #sg.client.mail.send.post(request_body=mail)

    print("==> send_text_email")
    return {"status": "success", "text": body}

instructions = "你能够根据一份详细报告发送格式良好的 HTML 邮件。系统会向你提供一份详细的报告。你应使用你的工具\
发送一封邮件，将该报告转换为整洁、美观的 HTML 格式，并设置一个合适的邮件主题。"

email_agent = Agent(
    name="Email agent",
    instructions=instructions,
    model=model,
    tools=[send_text_email],
)

#### 5. writer agent
instructions = """
你是一名高级研究员，负责针对一个研究问题撰写一份连贯的研究报告。
系统会向你提供原始研究问题，以及由研究助理完成的初步研究资料。
你应首先拟定报告大纲，描述报告的结构与逻辑安排。
随后，生成完整的研究报告并将其作为最终输出提交。
最终输出应采用 Markdown 格式，内容应详实、连贯，篇幅较长，目标为 5–10 页内容，至少 1000 字以上。
""".strip()

class ReportData(BaseModel):
    short_summary: str = Field(description="A short 2-3 sentence summary of the findings.")
    markdown_report: str = Field(description="The final report")
    follow_up_questions: list[str] = Field(description="Suggested topics to research further")

writer_agent = Agent(
    name="WriterAgent",
    instructions=instructions,
    model=model,
    output_type=ReportData,
)

#### 6. functions
async def plan_searches(query: str):
    """ Use the planner_agent to plan which searches to run for the query """
    print("--> Planning searches...")
    result = await Runner.run(planner_agent, f"Query: {query}")
    print(f"<-- Will perform {len(result.final_output.searches)} searches")
    return result.final_output

async def search(item: WebSearchItem):
    """ Use the search agent to run a web search for each item in the search plan """
    input = f"Search term: {item.query}\nReason for searching: {item.reason}"
    result = await Runner.run(search_agent, input)
    return result.final_output

async def perform_searches(search_plan: WebSearchPlan):
    """ Call search() for each item in the search plan """
    print("--> Searching...")
    tasks = [asyncio.create_task(search(item)) for item in search_plan.searches]
    results = await asyncio.gather(*tasks)
    print("<-- Finished searching")
    return results

async def write_report(query: str, search_results: list[str]):
    """ Use the writer agent to write a report based on the search results"""
    print("--> Thinking about report...")
    input = f"Original query: {query}\nSummarized search results: {search_results}"
    result = await Runner.run(writer_agent, input)
    print("<-- Finished writing report")
    return result.final_output

async def send_report_email(report: ReportData):
    """ Use the email agent to send an email with the report """
    print("--> Writing email...")
    result = await Runner.run(email_agent, report.markdown_report)
    print("<-- Email sent")
    return result

####
async def main():
    query ="2025 年最新的 AI 智能体（AI Agent）框架"

    with trace("Research trace"):
        print("==> Starting research...")
        search_plan = await plan_searches(query)
        search_results = await perform_searches(search_plan)
        report = await write_report(query, search_results)
        result = await send_report_email(report)
        print("<== Hooray!")

    email = json.loads(result.raw_responses[0].output[0].arguments)['body']

    with open(Path("data") / "lab4-2_research_email.html", 'w') as f:
        f.write(email)

    with open(Path("data") / "lab4-2_research_report.json", 'w') as f:
        f.write(report.model_dump_json(indent=2, ensure_ascii=False))

    with open(Path("data") / "lab4-2_research_report.md", 'w') as f:
        f.write(report.markdown_report)

if __name__ == "__main__":
    asyncio.run(main())
