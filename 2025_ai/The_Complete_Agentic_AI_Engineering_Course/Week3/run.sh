#!/bin/bash
set -eu -o pipefail; _wd=$(pwd); _dir=$(readlink -f `dirname "$0"`)


#### 1. create crew debat
pip install crewai
# uv tool install crewai

crewai create crew debate
# interactive
# - Select a provider to set up:
# - Enter the number of your choice or 'q' to quit:
# - Select a model to use for Openai:
# - Enter your OPENAI API key (press Enter to skip):

#### 2. uv install addtional packages
cd debate

#uv pip install -U httpx litellm
#uv pip install "requests[socks]" "httpx[socks]"

source .venv/bin/activate

#uv lock --upgrade-package litellm
uv add requests[socks] httpx[socks]
uv sync

#### 3. run
cat > .env <<EOF
#https_proxy=socks5h://127.0.0.1:1080

OPENAI_API_KEY=sk-xxxxxxxx
EOF

crewai run
