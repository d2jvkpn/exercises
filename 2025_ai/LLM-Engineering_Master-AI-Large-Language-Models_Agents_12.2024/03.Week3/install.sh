#!/bin/bash
set -eu -o pipefail; _wd=$(pwd); _dir=$(readlink -f `dirname "$0"`)


pip install -q --upgrade torch==2.5.1+cu124 torchvision==0.20.1+cu124 torchaudio==2.5.1+cu124 --index-url https://download.pytorch.org/whl/cu124
pip install -q --upgrade transformers==4.48.3 datasets==3.2.0 diffusers


pip show transformers datasets diffusers

pip install bitsandbytes sentencepiece


# pip install -U transformers accelerate torch
# pip install -U bitsandbytes sentencepiece

# pip cache dir
# pip cache purge
