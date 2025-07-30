#!/usr/bin/env python3
import os

from langchain.document_loaders import DirectoryLoader, TextLoader
from langchain.text_splitter import CharacterTextSplitter


# Read in documents using LangChain's loaders. Take everything in all the sub-folders of our knowledgebase
folders = glob.glob("data/knowledge-base/*")

text_loader_kwargs = {'encoding': 'utf-8'}

documents = []
for folder in folders:
    doc_type = os.path.basename(folder) # products, company, employees, contracts

    loader = DirectoryLoader(
        folder, glob="**/*.md", loader_cls=TextLoader, loader_kwargs=text_loader_kwargs,
    )

    folder_docs = loader.load()

    for doc in folder_docs:
        doc.metadata["doc_type"] = doc_type
        documents.append(doc)

print(documents[0])

text_splitter = CharacterTextSplitter(chunk_size=1000, chunk_overlap=200)
chunks = text_splitter.split_documents(documents)

doc_types = set(chunk.metadata['doc_type'] for chunk in chunks)
print(f"Document types found: {', '.join(doc_types)}")

for chunk in chunks:
    if 'CEO' in chunk.page_content:
        print(chunk)
        print("_________")
