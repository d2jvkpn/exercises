#!/usr/bin/env python3
import pickle

import chromadb
import pandas as pd
from tqdm import tqdm
from sentence_transformers import SentenceTransformer


#### chromadb
client = chromadb.PersistentClient(path="data/chromadb")
collection_name = "products"

existing_collection_names = client.list_collections()

if collection_name in existing_collection_names:
    client.delete_collection(collection_name)
    print(f"Deleted existing collection: {collection_name}")

collection = client.create_collection(collection_name)
collection = client.get_or_create_collection(collection_name)

model = SentenceTransformer('sentence-transformers/all-MiniLM-L6-v2')

####
def description(item):
    #text = item.prompt.replace("How much does this cost to the nearest dollar?\n\n", "")
    #return text.split("\n\nPrice is $")[0]
    text = item['prompt'].split("\n\n", 1)[1]

    return text.rsplit("\n\n", 1)[0]

with open('data/train.pkl', 'rb') as file:
    train = pickle.load(file)

for i in tqdm(range(0, len(train), 1000)):
    chunk = train[i: i+1000]

    documents = [description(item) for item in chunk]
    vectors = model.encode(documents).astype(float).tolist()
    metadatas = [{"category": item['category'], "price": item['price']} for item in chunk]

    ids = [f"doc_{j}" for j in range(i, i+len(documents))]

    collection.add(
        ids=ids,
        documents=documents,
        embeddings=vectors,
        metadatas=metadatas,
    )

result = collection.get(include=['embeddings', 'documents', 'metadatas'], limit=20_000)
vectors = np.array(result['embeddings'])
documents = result['documents']
categories = [metadata['category'] for metadata in result['metadatas']]
