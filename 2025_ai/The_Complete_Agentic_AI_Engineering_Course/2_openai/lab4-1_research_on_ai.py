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
instructions = "You are a research assistant. Given a search term, you search the web for that \
term and produce a concise summary of the results. The summary must 2-3 paragraphs and less than \
300 words. Capture the main points. Write succintly, no need to have complete sentences or good \
grammar. This will be consumed by someone synthesizing a report, so it's vital you capture the \
essence and ignore any fluff. Do not include any additional commentary other than the summary \
itself."

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
    query = "Latest AI Agent frameworks in 2025",
    save=Path("data") / "lab4-1_search_agent.json",
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

instructions = f"You are a helpful research assistant. Given a query, come up with a set of web \
searches to perform to best answer the query. Output {HOW_MANY_SEARCHES} terms to query for."

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
    query = "Latest AI Agent frameworks in 2025",
    save=Path("data") / "lab4-1_planner_agent.json",
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

instructions = "You are able to send a nicely formatted HTML email based on a detailed report. You \
will be provided with a detailed report. You should use your tool to send one email, providing the \
report converted into clean, well presented HTML with an appropriate subject line."

email_agent = Agent(
    name="Email agent",
    instructions=instructions,
    model=model,
    tools=[send_text_email],
)

#### 5. writer agent
instructions = """
You are a senior researcher tasked with writing a cohesive report for a research query.

You will be provided with the original query, and some initial research done by a research 
assistant.

You should first come up with an outline for the report that describes the structure and flow of 
the report. Then, generate the report and return that as your final output.

The final output should be in markdown format, and it should be lengthy and detailed. Aim for 
5-10 pages of content, at least 1000 words.
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
query ="Latest AI Agent frameworks in 2025"

with trace("Research trace"):
    print("==> Starting research...")
    search_plan = await plan_searches(query)
    search_results = await perform_searches(search_plan)
    report = await write_report(query, search_results)
    result = await send_report_email(report)
    print("<== Hooray!")

email = json.loads(result.raw_responses[0].output[0].arguments)['body']

with open(Path("data") / "lab4-1_research_email.html", 'w') as f:
    f.write(email)

with open(Path("data") / "lab4-1_research_report.json", 'w') as f:
    f.write(report.model_dump_json(indent=2, ensure_ascii=False))

with open(Path("data") / "lab4-1_research_report.md", 'w') as f:
    f.write(report.markdown_report)
