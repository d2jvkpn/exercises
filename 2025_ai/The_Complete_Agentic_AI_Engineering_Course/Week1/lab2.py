#!/usr/bin/env python3
import os, json
from pathlib import Path
from dotenv import load_dotenv
load_dotenv(Path("configs") / "local.env", override=True)

from openai import OpenAI


####
openai = OpenAI(
    api_key=os.getenv('OPENAI_API_KEY'),
    base_url=os.getenv("OPENAI_API_BASE"),
)

deepseek = OpenAI(
    api_key=os.getenv('DEEPSEEK_API_KEY'),
    base_url="https://api.deepseek.com/v1",
)


answers = []
competitors = []

question = """
Please come up with a challenging, nuanced question that I can ask a number of LLMs to evaluate their intelligence. "
Answer only with the question, no explanation.
""".replace("\n", "").strip()

messages = [{"role": "user", "content": question}]
print(f"==> question: {question}")

####
model_name = "gpt-4o-mini"
response = openai.chat.completions.create(model=model_name, messages=messages)
answer = response.choices[0].message.content

#from IPython.display import Markdown, display
#display(Markdown(answer))
answers.append(answer)
print(f"\n==> {model_name}: {answer}")

competitors.append(model_name)

#### ...
#model_name = "claude-3-7-sonnet-latest"
#model_name = "gemini-2.0-flash"

model_name = "deepseek-chat"
response = deepseek.chat.completions.create(model=model_name, messages=messages)
answer = response.choices[0].message.content

#display(Markdown(answer))
competitors.append(model_name)
answers.append(answer)
print(f"\n==> {model_name}: {answer}")

####
together = "\n\n".join([
    f"# Response from competitor {i+1}\n\n{v}"
    for i, v in enumerate(answers)
])

judge = f"""You are judging a competition between {len(competitors)} competitors.
Each model has been given this question:

{question}

Your job is to evaluate each response for clarity and strength of argument, and rank them in order of best to worst.
Respond with JSON, and only JSON, with the following format:
{{"results": ["best competitor number", "second best competitor number", "third best competitor number", ...]}}

Here are the responses from each competitor:

{together}

Now respond with the JSON with the ranked order of the competitors, nothing else. Do not include markdown formatting or code blocks."""

judge_messages = [{"role": "user", "content": judge}]


#### Judgement time!
response = openai.chat.completions.create(
    model="o3-mini",
    messages=judge_messages,
)
results = response.choices[0].message.content
print(results)


##### OK let's turn this into results!
results_dict = json.loads(results)
ranks = results_dict["results"]

print(f"\n==> Result: {results}")

for index, result in enumerate(ranks):
    competitor = competitors[int(result)-1]
    print(f"Rank {index+1}: {competitor}")
