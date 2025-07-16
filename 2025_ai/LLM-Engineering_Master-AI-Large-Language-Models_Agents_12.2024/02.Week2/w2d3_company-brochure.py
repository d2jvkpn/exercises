#!/usr/bin/env python3
import os, argparse, json
from datetime import datetime
from pathlib import Path
os.environ['LITELLM_LOCAL_MODEL_COST_MAP'] = "True"

import yaml, litellm
import gradio as gr
import requests
from bs4 import BeautifulSoup

parser = argparse.ArgumentParser(
    description="parse commandline arguments",
    formatter_class=argparse.ArgumentDefaultsHelpFormatter,
)

parser.add_argument("--config", help="config path", default=Path("configs") / "local.yaml")

parser.add_argument("--host", help="http listening host", default="127.0.0.1")
parser.add_argument("--port", help="http listening port", type=int, default=7861)
#parser.add_argument("--share", help="gradio share", action="store_true")
args = parser.parse_args()


with open(args.config, 'r') as f:
    llm = yaml.safe_load(f)

# print("~~~ llm:", llm)
# system_prompt = "You are a helpful assistant that responds in markdown"

class JSONLogger(gr.FlaggingCallback):
    def __init__(self, keys, log_dir="logs"):
        self.log_dir = Path(log_dir)
        self.keys = keys

        self.log_dir.mkdir(exist_ok=True)
        self.filepath = self.log_dir / "flagging.jsonl"
        self.file = None

    def setup(self, components, flagging_options):
        #print("~~~ Flagging setup")
        if not self.filepath.exists():
            self.file = open(self.filepath, "w", encoding="utf-8")
        else:
            self.file = open(self.filepath, "a", encoding="utf-8")

    def flag(self, flag_data, flag_option=None, username=None):
        timestamp = datetime.now().astimezone().strftime("%Y-%m-%dT%H:%M:%S%:z")

        record = {"timestamp": timestamp}
        if flag_option is not None:
            record["flag"] = flag_option

        for i, k in enumerate(self.keys):
            record[k] = flag_data[i]

        json.dump(record, self.file, ensure_ascii=False)
        self.file.write("\n")
        self.file.flush()

    def close(self):
        #print("~~~ Closing flagging callback and cleaning up")
        if self.file is not None:
            self.file.close()

class Website:
    """
    A utility class to represent a website that we have scraped
    """

    url: str
    title: str
    text: str

    def __init__(self, url):
        self.url = url
        response = requests.get(url)
        soup = BeautifulSoup(response.content, "html.parser")

        self.title = soup.title.string if soup.title else "No title found"
        for inrelevant in soup.body([
            "script", "style", "img", "input",
            "iframe", "meta", "noscript", "button"]):
            inrelevant.decompose()

        self.text = soup.body.get_text(separator="\n", strip=True)

    def get_contents(self):
        return f"Webpage title:\n{self.title}\n\nWebpage Contents:\n{self.text}\n"


# https://www.apple.com/
def message_gpt(model, url, stream=True):
    url = url.strip()
    if not url:
        return ""

    try:
        website = Website(url.strip())
    except Exception as e:
        yield f"Error: {e}"
        return

    prompt = "Please generate a company brochure. Here is their landing page:\n{}".format(
        website.get_contents(),
    )

    messages = [
        { "role": "system", "content": llm['system_prompt'] },
        { "role": "user", "content": prompt },
    ]

    provider = 'hosted_vllm' if llm.get('hosted_vllm') == True else llm.eg('provider')

    response = litellm.completion(
        custom_llm_provider=provider, model=llm['model'],
        api_base=llm['api_base'], api_key=llm.get('api_key'),
        max_tokens=llm['max_tokens'], temperature=llm['temperature'],
        num_retries=3, timeout=60, stream=stream,
        messages=messages,
    )

    if stream:
        reply_content = ""

        for chunk in response:
            delta = chunk.choices[0].delta.content or ""
            reply_content += delta
            yield reply_content
    else:
        yield response.choices[0].message.content


model_choices = ["openai/gpt", "anthropic/claude"]

view = gr.Interface(
    title=llm['title'],
    description=f"Parameters: temperature={llm['temperature']}, max_tokens={llm['max_tokens']}",
    fn=message_gpt,
    inputs=[
        gr.Dropdown(model_choices, label="Select model", value=model_choices[0]),
        gr.Textbox(label=f"Loading page URL:", lines=1),
    ],
    outputs=[
        gr.Textbox(label="Response:", lines=15),
    ],
    flagging_mode="auto",         # never, auto, manual
    #flagging_options=["A", "B"], # flagging_mode == "mannual"
    flagging_callback=JSONLogger(keys=["model", "message", "reply"]),
)

# https://fonts.googleapis.com/css2?family=Source+Sans+Pro:wght@400;600^&display=swap
# https://cdnjs.cloudflare.com/ajax/libs/iframe-resizer/4.3.1/iframeResizer.contentWindow.min.js
view.launch(share=False, server_name=args.host, server_port=args.port)
