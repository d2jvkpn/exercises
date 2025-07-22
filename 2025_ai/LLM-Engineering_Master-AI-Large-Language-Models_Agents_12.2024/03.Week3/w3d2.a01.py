#!/usr/bin/env python3

# https://colab.research.google.com/drive/1aMaEw8A56xs0bRM4lu8z7ou18jqyybGm?usp=sharing#scrollTo=sQ03dQDl2h0D

#### colab: Runtime -> Change runtime type -> T4 GPU
gpu_info = !nvidi-smi
print('\n'.join(gpu_info))


import subprocess
result = subprocess.run(['nvidia-smi'], capture_output=True, text=True)
print(f"Return Code: {result.returncode}\nOutput: {result.stdout}")

#result = subprocess.run(
#    ['grep', 'python'], input='hello\npython\nworld',
#    capture_output=True, text=True,
#)

#print(result.stdout)


!pip install -q --upgrade torch==2.5.1+cu124 torchvision==0.20.1+cu124 torchaudio==2.5.1+cu124 --index-url https://download.pytorch.org/whl/cu124
!pip install -q --upgrade transformers==4.48.3 datasets==3.2.0 diffusers

#!pip cache dir
#!pip cache purge


import torch
from google.colab import userdata
from huggingface_hub import login
from transformers import pipeline
from diffusers import DiffusionPipeline
from datasets import load_dataset
import soundfile as sf
from IPython.display import Audio

hf_token = userdata.get('HF_TOKEN')
login(hf_token, add_to_git_credential=True)


# Sentiment Analysis
classifier = pipeline("sentiment-analysis", device="cuda")
result = classifier("I'm super excited to be on the way to LLM mastery!")
print(result)
# [{'label': 'POSITIVE', 'score': 0.9993460774421692}]


# Named Entity Recognition
ner = pipeline("ner", grouped_entities=True, device="cuda")
result = ner("Barack Obama was the 44th president of the United States.")
print(result)
# [{'entity_group': 'PER', 'score': np.float32(0.99918306), 'word': 'Barack Obama', 'start': 0, 'end': 12}, {'entity_group': 'LOC', 'score': np.float32(0.9986908), 'word': 'United States', 'start': 43, 'end': 56}]

# Question Answering with Context
question_answerer = pipeline("question-answering", device="cuda")
result = question_answerer(
    question="Who was the 44th president of the United States?",
    context="Barack Obama was the 44th president of the United States.",
)
print(result)
# {'score': 0.9889456033706665, 'start': 0, 'end': 12, 'answer': 'Barack Obama'}


# Text Summarization
summarizer = pipeline("summarization", device="cuda")
text = """The Hugging Face transformers library is an incredibly versatile and powerful tool for 
natural language processing (NLP).
It allows users to perform a wide range of tasks such as text classification, named entity 
recognition, and question answering, among others.
It's an extremely popular library that's widely used by the open-source data science community.
It lowers the barrier to entry into the field by providing Data Scientists with a productive, 
convenient way to work with transformer models.
"""

summary = summarizer(text, max_length=50, min_length=25, do_sample=False)
print(summary[0]['summary_text'])


# Translation
translator = pipeline("translation_en_to_fr", device="cuda")
result = translator("The Data Scientists were truly amazed by the power and simplicity of the HuggingFace pipeline API.")
print(result[0]['translation_text'])


# Another translation, showing a model being specified
# All translation models are here: https://huggingface.co/models?pipeline_tag=translation&sort=trending
translator = pipeline("translation_en_to_es", model="Helsinki-NLP/opus-mt-en-es", device="cuda")
result = translator("The Data Scientists were truly amazed by the power and simplicity of the HuggingFace pipeline API.")
print(result[0]['translation_text'])


# Classification
classifier = pipeline("zero-shot-classification", device="cuda")
result = classifier("Hugging Face's Transformers library is amazing!", candidate_labels=["technology", "sports", "politics"])
print(result)


# Text Generation
generator = pipeline("text-generation", device="cuda")
result = generator("If there's one thing I want you to remember about using HuggingFace pipelines, it's")
print(result[0]['generated_text'])


# Image Generation
image_gen = DiffusionPipeline.from_pretrained(
    "stabilityai/stable-diffusion-2",
    torch_dtype=torch.float16,
    use_safetensors=True,
    variant="fp16"
    ).to("cuda")

text = "A class of Data Scientists learning about AI, in the surreal style of Salvador Dali"
image = image_gen(prompt=text).images[0]
image


# Audio Generation
synthesiser = pipeline("text-to-speech", "microsoft/speecht5_tts", device='cuda')
embeddings_dataset = load_dataset("Matthijs/cmu-arctic-xvectors", split="validation")
speaker_embedding = torch.tensor(embeddings_dataset[7306]["xvector"]).unsqueeze(0)
speech = synthesiser("Hi to an artificial intelligence engineer, on the way to mastery!", forward_params={"speaker_embeddings": speaker_embedding})

sf.write("speech.wav", speech["audio"], samplerate=speech["sampling_rate"])
Audio("speech.wav")


!ls ~/.cache/huggingface/hub
!du -sh ~/.cache/huggingface/hub/*

!rm -r ~/.cache/pip

!pip show transformers datasets diffusers
#4.53.2, 4.0.0, 0.34.0
