#! /usr/bin/env bash
set -eu -o pipefail # -x
_wd=$(pwd); _path=$(dirname $0 | xargs -i readlink -f {})


nmap -vv --max-retries=5 -sV -T4 -p- localhost
# -sV: Probe open ports to determine service/version info
# -T<0-5>: Set timing template (higher is faster)
# -p-: Only scan specified ports
#    Ex: -p22; -p1-65535; -p U:53,111,137,T:21-25,80,139,8080,S:9
# --exclude-ports <port ranges>: Exclude the specified ports from scanning

echo -e "\nalias quick_nmap='nmap -vv --max-retries=5 -sV -T4 -p-'" >> ~/.bashrc


