#! /usr/bin/env bash
set -eu -o pipefail
_wd=$(pwd)
_path=$(dirname $0 | xargs -i readlink -f {})

for m in yq jq curl; do
   command -v $m > /dev/null || { >&2 echo "command $m not found"; exit 1; }
done

save_to=~/.chatgpt/chatgp-requests
mkdir -p $save_to
# token=${ChatGPT_Token:-Your_Default_ChatGPT_API_Key}

[ -f ~/.chatgpt/env ] && source ~/.chatgpt/env
token=${ChatGPT_Token}

[ $# -eq 0 ] && { >&2 echo "Pass your question as argument(s)!"; exit 1; }
[ -z "${ChatGPT_Token}" ] && { >&2 echo "ChatGPT_Token is unset"; exit 1; }

question="$*"

tag=$(date +%FT%T-%s | sed 's/:/-/g')
echo ">>> $tag: $question"

ques_file=$save_to/${tag}_quesiton.json
ans_file=$save_to/${tag}_answer.json

jq -n \
  --arg model "${ChatGPT_Model:-gpt-3.5-turbo}" \
  --arg content "$question" \
  --argjson max_tokens "${ChatGPT_MaxTokens:-1024}" \
  --argjson temperature "${ChatGPT_Temperature:-1.0}" \
  '{model: $model, messages: [{"role": "user", "content": $content}], max_tokens: $max_tokens, temperature: $temperature}' > $ques_file


curl https://api.openai.com/v1/chat/completions \
  -x socks5h://localhost:1080              \
  -H 'Content-Type: application/json'      \
  -H "Authorization: Bearer $token"        \
  -d @$ques_file > $ans_file || { rm $ans_file; exit 1; }

jq -r .choices[].message.content $ans_file

{
  echo -e "\n#### QA"
  yq -P eval .  $ques_file
  echo -e "---"
  yq -P eval .  $ans_file
} >> $save_to/chatgpt_QA_$(date +%F).yaml

rm $ques_file $ans_file
