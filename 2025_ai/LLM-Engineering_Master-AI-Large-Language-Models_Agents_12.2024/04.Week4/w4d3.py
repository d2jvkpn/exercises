#!/usr/bin/env python3
import os, glob

from dotenv import load_dotenv
import gradio as gr
from langchain.document_loaders import DirectoryLoader, TextLoader
from langchain.text_splitter import CharacterTextSplitter, TokenTextSplitter
from langchain.schema import Document
from langchain_openai import OpenAIEmbeddings, ChatOpenAI
from langchain_chroma import Chroma
from langchain.vectorstores import FAISS
from langchain.chains import ConversationalRetrievalChain
from langchain_core.callbacks import StdOutCallbackHandler
from langchain_core.runnables import Runnable
from langchain.memory import ChatMessageHistory, ConversationBufferMemory
#import numpy as np
#from sklearn.manifold import TSNE
#import plotly.graph_objects as go


#### 1. init
load_dotenv("configs/local.env", override=True)
assert(os.getenv('OPENAI_API_KEY') is not None)

model = "gpt-4o-mini"
db_name = "data/chroma"

#### 2. documents
folders = glob.glob("data/knowledge-base/*")

def add_metadata(doc, doc_type):
    doc.metadata["doc_type"] = doc_type
    return doc

documents = []
for folder in folders:
    doc_type = os.path.basename(folder)

    loader = DirectoryLoader(
        folder, glob="**/*.md",
        loader_cls=TextLoader,
        loader_kwargs={'encoding': 'utf-8'},
    )

    folder_docs = loader.load()
    documents.extend([add_metadata(doc, doc_type) for doc in folder_docs])

    continue
    for doc in folder_docs:
        doc.metadata["doc_type"] = doc_type
        documents.append(doc)

# TokenTextSplitter(chunk_size=1000, chunk_overlap=200,  #encoding_name="cl100k_base")
text_splitter = CharacterTextSplitter(
    chunk_size=1000, chunk_overlap=200,
)

chunks = text_splitter.split_documents(documents)
doc_types = set(chunk.metadata['doc_type'] for chunk in chunks)
print(f"Document types found: {', '.join(doc_types)}")


#### 3. Chroma vector database
embeddings = OpenAIEmbeddings()

if os.path.exists(db_name):
    Chroma(persist_directory=db_name, embedding_function=embeddings).delete_collection()

vectorstore = Chroma.from_documents(
    documents=chunks, embedding=embeddings, persist_directory=db_name,
)

print(f"Vectorstore created with {vectorstore._collection.count()} documents")


#### 4. embedding
collection = vectorstore._collection
sample_embedding = collection.get(limit=1, include=["embeddings"])["embeddings"][0]
dimensions = len(sample_embedding)
print(f"The vectors have {dimensions:,} dimensions")

# vectorstore = FAISS.from_documents(chunks, embedding=embeddings) # FAISS vector database
# total_vectors = vectorstore.index.ntotal
# dimensions = vectorstore.index.d
# print(f"There are {total_vectors} vectors with {dimensions:,} dimensions in the vector store")

#### 5. chat
# create a new Chat with OpenAI
llm = ChatOpenAI(temperature=0.7, model_name=model)

# set up the conversation memory for the chat
# LangChainDeprecationWarning: Please see the migration guide at: https://python.langchain.com/docs/versions/migrating_memory/
memory = ConversationBufferMemory(memory_key='chat_history', return_messages=True)

# the retriever is an abstraction over the VectorStore that will be used during RAG
retriever = vectorstore.as_retriever()

conversation_chain = ConversationalRetrievalChain.from_llm(
    llm=llm, retriever=retriever, memory=memory, callbacks=[StdOutCallbackHandler()],
)

query = "Can you describe Insurellm in a few sentences"
result = conversation_chain({"question": query})
print(result["answer"])

#### 6. gradio
# putting it together: set up the conversation chain with the GPT 3.5 LLM, the vector store and memory
conversation_chain = ConversationalRetrievalChain.from_llm(
   llm=llm, retriever=retriever, memory=memory,
)

# Wrapping that in a function
def chat(message, history):
    result = conversation_chain.invoke({"question": message})
    return result["answer"]

view = gr.ChatInterface(chat, type="messages").launch(inbrowser=True)
