#!/usr/bin/env python3
# pip install --upgrade --quiet langchain langchain-community langchain_openai

import os
from pathlib import Path

from dotenv import load_dotenv
load_dotenv(Path("configs") / "local.env")
from langchain_community.utilities import GoogleSerperAPIWrapper


#os.getenv("OPENAI_API_KEY")
#os.getenv("SERPER_API_KEY")
search = GoogleSerperAPIWrapper()

#---
result = search.run("Obama's first name?")
print(result)

#---
result = search.results("OpenAI latest news", num_results=5)
# ['searchParameters', 'organic', 'topStories', 'peopleAlsoAsk', 'relatedSearches', 'credits']

print("results:")
for v in result['organic']:
    print(f"- link: {v['link']}")
    print(f"  title: {v['title']}")
    print(f"  snippet: {v['snippet']}")
    print(f"  source: {v.get('source', '')}")
    print(f"  date: {v.get('date', '')}")

#---
search_news = GoogleSerperAPIWrapper(type="news", gl="us", hl="en")
news = search_news.results("AI chips market", num_results=3)

#---
search_news = GoogleSerperAPIWrapper(
    type="news", location="China", gl="cn", hl="zh-cn", tbs="qdr:w",
)

news = search_news.results("小米汽车", num_results=3)
