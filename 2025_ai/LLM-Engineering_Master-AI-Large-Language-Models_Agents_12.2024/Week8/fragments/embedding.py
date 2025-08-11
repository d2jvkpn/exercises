#!/usr/bin/env python3

import numpy as np
from sentence_transformers import SentenceTransformer


def cosine_similarity(a, b):
    return np.dot(a, b) / (np.linalg.norm(a) * np.linalg.norm(b))

def how_similar(text1, text2):
    vector1, vector2 = model.encode([text1, text2])
    similarity = cosine_similarity(vector1, vector2)

    print(f"Similarity between {text1} and {text2} is {similarity*100:.1f}%")

    return similarity

model = SentenceTransformer('sentence-transformers/all-MiniLM-L6-v2')

vector = model.encode(["Well hi there"])[0]

similarity = cosine_similarity(vector1, vector2)
