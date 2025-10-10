#!/usr/bin/env python3
import os, asyncio
from pathlib import Path
from typing import Dict
from dotenv import load_dotenv
load_dotenv(Path("configs") / "local.env", override=True)

from openai import AsyncOpenAI
from openai.types.responses import ResponseTextDeltaEvent
from agents import Agent, Runner, function_tool # trace
from agents import set_default_openai_client, set_tracing_disabled # set_default_openai_api
#import sendgrid
#from sendgrid.helpers.mail import Mail, Email, To, Content

#### 1. init
llm_client = AsyncOpenAI(
    base_url=os.getenv("OPENAI_BASE_URL", "https://api.openai.com/v1"),
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

#### 2. three sales agents
instructions1 = "你是一名为 ComplAI 公司工作的销售代表。该公司提供一款由 AI 驱动的 SaaS 工具，用于确保 SOC2 \
合规 并帮助企业准备审计。你撰写专业且正式的冷启动销售邮件。"

sales_agent1 = Agent(name="Professional Sales Agent", instructions=instructions1, model=model)


instructions2 = "你是一名幽默、富有吸引力的销售代表，供职于 ComplAI，该公司提供一款由 AI 驱动的 SaaS 工具，用\
于确保 SOC2 合规 并准备审计。你撰写机智、有趣、能引起回复的冷启动销售邮件。"

sales_agent2 = Agent(name="Engaging Sales Agent", instructions=instructions2, model=model)


instructions3 = "你是一名忙碌的销售代表，供职于 ComplAI，该公司提供一款由 AI 驱动的 SaaS 工具，用于确保 SOC2 \
合规 并准备审计。你撰写简洁、直截了当的冷启动销售邮件。"

sales_agent3 = Agent(name="Busy Sales Agent", instructions=instructions3, model=model)

async def test01_sales_agent1():
    result = Runner.run_streamed(sales_agent1, input="撰写一封冷启动销售邮件")

    output = ""
    async for event in result.stream_events():
        if event.type == "raw_response_event" and isinstance(event.data, ResponseTextDeltaEvent):
            output += event.data.delta
            #print(event.data.delta, end="", flush=True)

    return result.final_output

async def test02_write_cold_emails():
    message = "撰写一封冷启动销售邮件"

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
instructions = "你需要从给定的选项中选择最好的冷启动销售邮件。假设你是一位客户，选择你最有可能回复的一封。不要解\
释原因，只需回复所选邮件内容即可。"

sales_picker = Agent(name="Sales Picker", instructions=instructions, model=model)

async def test03_write_a_cold_email():
    message = "撰写一封冷启动销售邮件"

    results = await asyncio.gather(
        Runner.run(sales_agent1, input=message),
        Runner.run(sales_agent2, input=message),
        Runner.run(sales_agent3, input=message),
    )

    outputs = [result.final_output for result in results]
    emails = "冷启动销售邮件:\n\n" + "\n\邮件:\n\n".join(outputs)

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

    print(f"--> send_html_email")
    return {"status": "success", "text": html_body}

#### 5. tools
# ----
description = "撰写一封冷启动销售邮件"
tool1 = sales_agent1.as_tool(tool_name="sales_agent1", tool_description=description)
tool2 = sales_agent2.as_tool(tool_name="sales_agent2", tool_description=description)
tool3 = sales_agent3.as_tool(tool_name="sales_agent3", tool_description=description)

# ----
instructions = "你可以为一封冷启动销售邮件撰写主题。系统会提供一封邮件内容，你需要为其撰写一个能够提高回复率的邮\
件主题。"

subject_writer = Agent(name="Email subject writer", instructions=instructions, model=model)

subject_tool = subject_writer.as_tool(
    tool_name="subject_writer",
    tool_description="撰写一封冷启动销售邮件的主题",
)

# ----
instructions = "你可以将一封纯文本邮件正文转换为 HTML 格式的邮件正文。系统会提供一份可能包含 Markdown 的文本邮\
件正文，你需要将其转换为简洁、清晰且具有吸引力的 HTML 邮件版式与设计。"

html_converter = Agent(name="HTML email body converter", instructions=instructions, model=model)

html_tool = html_converter.as_tool(
    tool_name="html_converter",
    tool_description="Convert a text email body to an HTML email body",
)

instructions ="你是一名邮件排版与发送专员。你将收到一封待发送邮件的正文。你需要先使用 subject_writer 工具为该\
邮件撰写主题，然后使用 html_converter 工具将正文转换为 HTML 格式。最后，使用 send_html_email 工具将带有主题\
和 HTML 正文的邮件发送出去。"

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
你是 ComplAI 的销售经理。你的目标是使用 sales_agent 工具 找出最优秀的一封冷启动销售邮件。

请严格按照以下步骤操作：
1. 生成草稿： 使用全部三个 sales_agent 工具 生成三封不同的邮件草稿。在三封草稿都准备好之前，不得进入下一步。
2. 评估与选择： 审阅这三封草稿，并根据你的判断选择一封最有效、最有可能获得回复的邮件。
如果对初次生成的结果不满意，可以多次使用这些工具。
3. 交接发送： 仅将唯一选中的邮件草稿交给 “Email Manager” 智能体。该智能体将负责邮件的排版与发送。

关键规则：
- 你必须使用 sales_agent 工具 来生成邮件草稿，不得自行撰写。
- 你只能交接一封且仅一封邮件给 Email Manager，绝不能多于一封。
"""

sales_manager = Agent(
    name="Sales Manager",
    instructions=sales_manager_instructions,
    model=model,
    tools=[tool1, tool2, tool3],
    handoffs=[emailer_agent],
)

async def test04_sales_manager():
    message = "发送一封以 “Dear CEO” 开头、由 Alice 发出的冷启动销售邮件。"
    result = await Runner.run(sales_manager, message)

    return result

#### 7. test
# ----
result = asyncio.run(test01_sales_agent1())

with open(Path("data") / 'lab2-2_sales_agent1.txt', 'w') as f:
    f.write(result)

# ----
result = asyncio.run(test02_write_cold_emails())

with open(Path("data") / 'lab2-2_write_cold_emails.txt', 'w') as f:
    f.write(result)

# ----
result = asyncio.run(test03_write_a_cold_email())

with open(Path("data") / 'lab2-2_write_a_cold_email.txt', 'w') as f:
    f.write(result)

# ----
result = asyncio.run(test04_sales_manager())

with open(Path("data") / 'lab2-2_sales_manager.txt', 'w') as f:
    f.write(result.final_output)
