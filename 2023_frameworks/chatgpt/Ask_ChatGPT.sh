#! /usr/bin/env bash
set -eu -o pipefail
_wd=$(pwd)
_path=$(dirname $0 | xargs -i readlink -f {})

mkdir -p chatgp-requests
# token=${ChatGPT_Token:-Your_Default_ChatGPT_API_Key}

[ -f ~/.chatgpt/env ] && source ~/.chatgpt/env
token=${ChatGPT_Token}
# curl https://api.openai.com/v1/models -H "Authorization: Bearer $token" > chatgp-requests/ChatGPT_models.json

[[ $# -eq 0 ]] && { >&2 echo "Pass your question as argument(s)!"; exit 1; }
question=$*

tag=$(date +%FT%T-%s | sed 's/:/-/g')
echo ">>> $tag: $question"

ques_file=chatgp-requests/${tag}_quesiton.json
ans_file=chatgp-requests/${tag}_answer.json

cat > $ques_file <<EOF
{
  "model": "${ChatGPT_Model:-text-davinci-003}",
  "prompt": "$question",
  "max_tokens": ${ChatGPT_MaxTokens:-2048},
  "temperature": ${ChatGPT_Temperature:-1.0}
}
EOF

curl https://api.openai.com/v1/completions \
  -H 'Content-Type: application/json'      \
  -H "Authorization: Bearer $token"        \
  -d @$ques_file > $ans_file || { rm $ans_file; }

jq -r .choices[0].text $ans_file

{
  echo -e "\n#### QA"
  yq -P eval .  $ques_file
  echo -e "---"
  yq -P eval .  $ans_file
} >> chatgp-requests/chatgpt_QA_$(date +%F).yaml

rm $ques_file $ans_file
