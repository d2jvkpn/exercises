#!/usr/bin/env python3
import os, asyncio
from typing import Dict
from pathlib import Path
from dotenv import load_dotenv
load_dotenv(Path("configs") / "local.env", override=True)

from openai import AsyncOpenAI
from openai.types.responses import ResponseTextDeltaEvent
from agents import Agent, Runner, function_tool, trace
from agents import OpenAIChatCompletionsModel, input_guardrail, GuardrailFunctionOutput
from agents import set_default_openai_client, set_tracing_disabled # set_default_openai_api
#import sendgrid
#from sendgrid.helpers.mail import Mail, Email, To, Content
from pydantic import BaseModel

#### 1. init
llm_client = AsyncOpenAI(
    base_url=os.getenv("OPENAI_BASE_URL", "https://api.openai.com/v1"),
    api_key=os.getenv("OPENAI_API_KEY"),
)

model = "gpt-4o-mini"

set_tracing_disabled(True)
set_default_openai_client(llm_client)
#set_default_openai_api("chat_completions")

#### 2. sale agents
instructions1 = "You are a sales agent working for ComplAI, a company that provides a SaaS tool \
for ensuring SOC2 compliance and preparing for audits, powered by AI. You write professional, \
serious cold emails."

instructions2 = "You are a humorous, engaging sales agent working for ComplAI, a company that \
provides a SaaS tool for ensuring SOC2 compliance and preparing for audits, powered by AI. You \
write witty, engaging cold emails that are likely to get a response."

instructions3 = "You are a busy sales agent working for ComplAI, a company that provides a SaaS \
tool for ensuring SOC2 compliance and preparing for audits, powered by AI. You write concise, to \
the point cold emails."

deepseek_client = AsyncOpenAI(
    base_url="https://api.deepseek.com/v1",
    api_key=os.getenv("DEEPSEEk_API_KEY"),
)

deepseek_model = OpenAIChatCompletionsModel(model="deepseek-chat", openai_client=deepseek_client)
sales_agent = Agent(name="DeepSeek Sales Agent", instructions=instructions1, model=deepseek_model)

openai_model = OpenAIChatCompletionsModel(model="gpt-4o-mini", openai_client=llm_client)
openai_agent = Agent(name="DeepSeek Sales Agent", instructions=instructions1, model=openai_model)

description = "Write a cold sales email"
tool1 = sales_agent.as_tool(tool_name="sales_agent1", tool_description=description)
tool2 = openai_agent.as_tool(tool_name="sales_agent2", tool_description=description)

#### 3. function tools
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

@function_tool
def send_html_email(subject: str, html_body: str) -> Dict[str, str]:
    """ Send out an email with the given subject and HTML body to all sales prospects """
    #sg = sendgrid.SendGridAPIClient(api_key=os.environ.get('SENDGRID_API_KEY'))
    #from_email = Email("ed@edwarddonner.com")  # Change to your verified sender
    #to_email = To("ed.donner@gmail.com")  # Change to your recipient
    #content = Content("text/html", html_body)
    #mail = Mail(from_email, to_email, subject, content).get()
    #sg.client.mail.send.post(request_body=mail)

    print(f"==> send_html_email")
    return {"status": "success", "text": html_body}

#### 4. emailer agent
# ----
instructions = "You can write a subject for a cold sales email. You are given a message and you \
need to write a subject for an email that is likely to get a response."

subject_writer = Agent(name="Email subject writer", instructions=instructions, model=model)

subject_tool = subject_writer.as_tool(
    tool_name="subject_writer",
    tool_description="Write a subject for a cold sales email",
)

# ----
instructions = "You can convert a text email body to an HTML email body. You are given a text \
email body which might have some markdown and you need to convert it to an HTML email body with \
simple, clear, compelling layout and design."

html_converter = Agent(name="HTML email body converter", instructions=instructions, model=model)

html_tool = html_converter.as_tool(
    tool_name="html_converter",
    tool_description="Convert a text email body to an HTML email body",
)

# ----
instructions ="You are an email formatter and sender. You receive the body of an email to be sent. \
You first use the subject_writer tool to write a subject for the email, then use the \
html_converter tool to convert the body to HTML. Finally, you use the send_html_email tool to send \
the email with the subject and HTML body."

emailer_agent = Agent(
    name="Email Manager",
    instructions=instructions,
    model=model,
    tools=[subject_tool, html_tool, send_html_email],
    handoff_description="Convert an email to HTML and send it",
)

#### 5. sales manager
sales_manager_instructions = """
You are a Sales Manager at ComplAI. Your goal is to find the single best cold sales email using 
the sales_agent tools.
 
Follow these steps carefully:
1. Generate Drafts: Use all three sales_agent tools to generate three different email drafts. Do 
not proceed until all three drafts are ready.
 
2. Evaluate and Select: Review the drafts and choose the single best email using your judgment of 
which one is most effective. You can use the tools multiple times if you're not satisfied with the 
results from the first try.
 
3. Handoff for Sending: Pass ONLY the winning email draft to the 'Email Manager' agent. The Email 
Manager will take care of formatting and sending.
 
Crucial Rules:
- You must use the sales agent tools to generate the drafts — do not write them yourself.
- You must hand off exactly ONE email to the Email Manager — never more than one.
"""

sales_manager = Agent(
    name="Sales Manager",
    instructions=sales_manager_instructions,
    model=model,
    tools=[tool1, tool2],
    handoffs=[emailer_agent],
)

message = "Send out a cold sales email addressed to Dear CEO from Alice"

with trace("Automated SDR"):
    result = await Runner.run(sales_manager, message)
    with open(Path("data") / 'lab3_sales_manager.txt', 'w') as f:
        f.write(result.final_output)

#### 6. guardrail agent
class NameCheckOutput(BaseModel):
    is_name_in_message: bool
    name: str

guardrail_agent = Agent(
    name="Name check",
    instructions="Check if the user is including someone's personal name in what they want you to do.",
    model=model,
    output_type=NameCheckOutput,
)

@input_guardrail
async def guardrail_against_name(ctx, agent, message):
    result = await Runner.run(guardrail_agent, message, context=ctx.context)
    is_name_in_message = result.final_output.is_name_in_message
    print(f"!!! guardrail_against_name: {is_name_in_message}")

    return GuardrailFunctionOutput(
        output_info={ "found_name": result.final_output },
        tripwire_triggered=is_name_in_message,
    )

careful_sales_manager = Agent(
    name="Sales Manager",
    instructions=sales_manager_instructions,
    model=model,
    tools=[tool1, tool2],
    handoffs=[emailer_agent],
    input_guardrails=[guardrail_against_name]
)

message = "Send out a cold sales email addressed to Dear CEO from Alice"

with trace("Protected Automated SDR"):
    try:
        result = await Runner.run(careful_sales_manager, message)
        with open(Path("data") / "lab3_careful_sales_manager.txt", 'w') as f:
            f.write(result.final_output)
    except Exception as e:
        print(f"!!! Got an exception: {e}")
