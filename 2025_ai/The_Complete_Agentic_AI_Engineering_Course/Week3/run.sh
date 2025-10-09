#!/bin/bash
set -eu -o pipefail; _wd=$(pwd); _dir=$(readlink -f `dirname "$0"`)


####
pip install crewai

# uv tool install crewai

####
crewai create crew debate
# interactive
# - Select a provider to set up:
# - Enter the number of your choice or 'q' to quit:
# - Select a model to use for Openai:
# - Enter your OPENAI API key (press Enter to skip):

cd debate

crewai uv sync

#uv pip install -U httpx litellm
#uv pip install "requests[socks]" "httpx[socks]"

uv lock --upgrade-package litellm
uv add requests[socks] httpx[socks]


uv sync
