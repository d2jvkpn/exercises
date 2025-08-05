#!/usr/bin/env python3
import os

import dotenv
from huggingface_hub import snapshot_download # login

repo_id = os.sys.argv[1]

dotenv.load_dotenv("configs/local.env")
#login(hf_token, add_to_git_credential=True)

cache_dir = "data/huggingface" # ~/.cache/huggingface/hub
os.makedirs(cache_dir, exist_ok=True)

snapshot_download(repo_id=repo_id, cache_dir=cache_dir)
