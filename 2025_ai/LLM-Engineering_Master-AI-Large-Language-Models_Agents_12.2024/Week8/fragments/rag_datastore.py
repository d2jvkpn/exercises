#!/usr/bin/env python3
import pickle

import numpy as np
import pandas as pd
import chromadb
from tqdm import tqdm
from sentence_transformers import SentenceTransformer


#### chromadb
client = chromadb.PersistentClient(path="data/chromadb")
collection_name = "products"

#existing_collection_names = client.list_collections()

#if collection_name in existing_collection_names:
#    client.delete_collection(collection_name)
#    print(f"Deleted existing collection: {collection_name}")

#collection = client.create_collection(collection_name)
collection = client.get_or_create_collection(collection_name)

#model = SentenceTransformer('sentence-transformers/all-MiniLM-L6-v2')
model = SentenceTransformer('data/all-MiniLM-L6-v2')

####
def description(item):
    #text = item.prompt.replace("How much does this cost to the nearest dollar?\n\n", "")
    #return text.split("\n\nPrice is $")[0]
    text = item['prompt'].split("\n\n", 1)[1]

    return text.rsplit("\n\n", 1)[0]

with open('data/train.pkl', 'rb') as file:
    train = pickle.load(file)
    print(f"train: {train.shape}")

for i in tqdm(range(0, len(train), 1000)):
    chunk = train.iloc[i: i+1000]

    documents = chunk.apply(description, axis=1).to_list()
    vectors = model.encode(documents).astype(float).tolist()

    metadatas = chunk.apply(
        lambda row: {'category': row['category'], 'price': row['price']}, axis=1,
    ).tolist()

    ids = [f"doc_{j+1}" for j in range(i, i+len(documents))]

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


def find_similars(text):
    vector = model.encode(text)
    results = collection.query(query_embeddings=vector, n_results=5)
    documents = results['documents'][0][:]
    prices = [m['price'] for m in results['metadatas'][0][:]]

    return documents, prices

documents, prices = find_similars("Quadcast HyperX condenser mic for high quality podcasting")
