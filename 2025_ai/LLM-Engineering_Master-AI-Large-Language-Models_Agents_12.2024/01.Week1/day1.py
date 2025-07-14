#!/usr/bin/env python3
import os

import requests
from dotenv import load_dotenv
from bs4 import BeautifulSoup
from openai import OpenAI
# from IPython.display import Markdown, display


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


SYTEME_PROMPT = """You are an assistant that anaylyzes the contents of a website and provides a
short summary, ignoring text that might be navigation related. Response in markdown.
"""

USER_PROMPT = """You are looking at a website titled {}. The contents of this website is as follows;
please provide a short summary of this website in markdown. If it includes news for announcements, 
then summarize these too.

{}
"""


def summarize_website(client: OpenAI, url: str):
    website = Website(url)

    messages = [
        {"role": "system", "content": SYTEME_PROMPT },
        {"role": "user", "content": USER_PROMPT.format(repr(website.title), website.text)}
    ]

    response = client.chat.completions.create(
        model = "gpt-4o-mini",
        messages = messages,
    )

    content = response.choices[0].message.content
    # display(Markdown(content))
    print(f"\n======== {url} ========\n{content}\n----------------\n")


load_dotenv()

openai = OpenAI(api_key=os.getenv("LLM_API_KEY"))

summarize_website(openai, "https://edwarddonner.com")
summarize_website(openai, "https://cnn.com")
summarize_website(openai, "https://anthropic.com")
