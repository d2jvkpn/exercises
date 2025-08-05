#!/usr/bin/env python3
import os
os.environ["HF_HOME"] = "data/huggingface"

import dotenv
from huggingface_hub import snapshot_download # login

dotenv.load_dotenv("configs/local.env")
#login(hf_token, add_to_git_credential=True)


repo_id = os.sys.argv[1]
snapshot_download(repo_id=repo_id)
