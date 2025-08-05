
#### colab: gpu=T4, Python 3.11.13
pip install -q --upgrade torch==2.5.1+cu124 --index-url https://download.pytorch.org/whl/cu124
# torchvision==0.20.1+cu124 torchaudio==2.5.1+cu124
pip install -q huggingface_hub datasets requests peft bitsandbytes transformers==4.48.3 accelerate==1.3.0 sentencepiece

#### python3.12
pip install -q --upgrade torch --index-url https://download.pytorch.org/whl/cu126
# torchvision torchaudio

pip install huggingface_hub datasets requests maptplotlib

pip install transformers sentence-transformers \
    peft bitsandbytes accelerate sentencepiece wandb trl
