#!/usr/bin/env python3
import os, asyncio
from pathlib import Path
from typing import Dict
from dotenv import load_dotenv
load_dotenv(Path("configs") / "local.env", override=True)

from openai import AsyncOpenAI
from openai.types.responses import ResponseTextDeltaEvent
from agents import Agent, Runner, function_tool # trace
from agents import set_default_openai_client, set_default_openai_api, set_tracing_disabled
#import sendgrid
#from sendgrid.helpers.mail import Mail, Email, To, Content

#### 1. init
llm_client = AsyncOpenAI(
    base_url=os.getenv("OPENAI_API_BASE", "https://api.openai.com/v1"),
    api_key=os.getenv("OPENAI_API_KEY"),
)

model = "gpt-4o-mini"

set_tracing_disabled(True)
set_default_openai_client(llm_client)
#set_default_openai_api("chat_completions")

# Let's just check emails are working for you

#def send_test_email():
#    sg = sendgrid.SendGridAPIClient(api_key=os.environ.get('SENDGRID_API_KEY'))
#    from_email = Email("ed@edwarddonner.com")  # Change to your verified sender
#    to_email = To("ed.donner@gmail.com")  # Change to your recipient
#    content = Content("text/plain", "This is an important test email")
#    mail = Mail(from_email, to_email, "Test email", content).get()
#    response = sg.client.mail.send.post(request_body=mail)
#    print(response.status_code)

#send_test_email()

#### 2. sales agents
instructions1 = "You are a sales agent working for ComplAI, a company that provides a SaaS tool \
for ensuring SOC2 compliance and preparing for audits, powered by AI. You write professional, \
serious cold emails."

sales_agent1 = Agent(name="Professional Sales Agent", instructions=instructions1, model=model)


instructions2 = "You are a humorous, engaging sales agent working for ComplAI, a company that \
provides a SaaS tool for ensuring SOC2 compliance and preparing for audits, powered by AI. You \
write witty, engaging cold emails that are likely to get a response."

sales_agent2 = Agent(name="Engaging Sales Agent", instructions=instructions2, model=model)


instructions3 = "You are a busy sales agent working for ComplAI, a company that provides a SaaS \
tool for ensuring SOC2 compliance and preparing for audits, powered by AI. You write concise, to \
the point cold emails."

sales_agent3 = Agent(name="Busy Sales Agent", instructions=instructions3, model=model)


async def test01_sales_agent1():
    result = Runner.run_streamed(sales_agent1, input="Write a cold sales email")

    output = ""
    async for event in result.stream_events():
        if event.type == "raw_response_event" and isinstance(event.data, ResponseTextDeltaEvent):
            output += event.data.delta
            #print(event.data.delta, end="", flush=True)

    return output

async def test02_write_cold_emails():
    message = "Write a cold sales email"

    results = await asyncio.gather(
        Runner.run(sales_agent1, input=message),
        Runner.run(sales_agent2, input=message),
        Runner.run(sales_agent3, input=message),
    )

    outputs = [result.final_output for result in results]

    #for i, output in enumerate(outputs):
    #    print(f"==> output: {i}")
    #    print(output + "\n\n")
    return "\n\n".join(outputs)

#### 3. sales picker agent
instructions = "You pick the best cold sales email from the given options. Imagine you are a \
customer and pick the one you are most likely to respond to. Do not give an explanation; reply \
with the selected email only.",

sales_picker = Agent(name="Sales Picker", instructions=instructions, model=model)

async def test03_write_a_cold_email():
    message = "Write a cold sales email"

    results = await asyncio.gather(
        Runner.run(sales_agent1, input=message),
        Runner.run(sales_agent2, input=message),
        Runner.run(sales_agent3, input=message),
    )

    outputs = [result.final_output for result in results]
    emails = "Cold sales emails:\n\n" + "\n\nEmail:\n\n".join(outputs)

    best = await Runner.run(sales_picker, input=emails)

    #print(f"Best sales email:\n{best.final_output}")
    return best.final_output

#### 4. functions
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

#### 5. tools
description = "Write a cold sales email"
tool1 = sales_agent1.as_tool(tool_name="sales_agent1", tool_description=description)
tool2 = sales_agent2.as_tool(tool_name="sales_agent2", tool_description=description)
tool3 = sales_agent3.as_tool(tool_name="sales_agent3", tool_description=description)

#tools = [tool1, tool2, tool3, send_text_email]

subject_instructions = "You can write a subject for a cold sales email. You are given a message \
and you need to write a subject for an email that is likely to get a response."

subject_writer = Agent(name="Email subject writer", instructions=subject_instructions, model=model)

subject_tool = subject_writer.as_tool(
    tool_name="subject_writer",
    tool_description="Write a subject for a cold sales email",
)


html_instructions = "You can convert a text email body to an HTML email body. You are given a text \
email body which might have some markdown and you need to convert it to an HTML email body with \
simple, clear, compelling layout and design."


html_converter = Agent(
    name="HTML email body converter",
    instructions=html_instructions,
    model=model,
)

html_tool = html_converter.as_tool(
    tool_name="html_converter",
    tool_description="Convert a text email body to an HTML email body",
)

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

#### 6. sales manager
# Improved instructions thanks to student Guillermo F.

sales_manager_instructions = """
You are a Sales Manager at ComplAI. Your goal is to find the single best cold sales email using the 
sales_agent tools.

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
    tools=[tool1, tool2, tool3],
    handoffs=[emailer_agent],
)

async def test04_sales_manager():
    message = "Send out a cold sales email addressed to Dear CEO from Alice"
    result = await Runner.run(sales_manager, message)

    return result

#### 7. test
result = asyncio.run(test01_sales_agent1())

with open(Path("data") / 'lab2_test01_sales_agent1.txt', 'w') as f:
    f.write(result)

result = asyncio.run(test02_write_cold_emails())

with open(Path("data") / 'lab2_test02_write_cold_emails.txt', 'w') as f:
    f.write(result)

result = asyncio.run(test03_write_a_cold_email())

with open(Path("data") / 'lab2_test03_write_a_cold_email.txt', 'w') as f:
    f.write(result)

result = asyncio.run(test04_sales_manager())

with open(Path("data") / 'lab2_test04_sales_manager.txt', 'w') as f:
    f.write(result.final_output)
