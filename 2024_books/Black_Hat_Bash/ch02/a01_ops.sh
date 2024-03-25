#! /usr/bin/env bash
set -eu -o pipefail # -x
_wd=$(pwd); _path=$(dirname $0 | xargs -i readlink -f {})


cat <<EOF
#### test files
- -d, -f, -s
- -r, -w, -x

#### operator
- =,  ==, !=
- <, >
- -z, -n

#### comparison
- -eq, -ne
- -gt, -lt
- -ge, le

```bash
if [[ "${VARIABLE_ONE}" == "${VARIABLE_TWO}" ]]; then
    >&2 echo "They are equal!"
else
    echo "They are not equal!"
fi

echo "Hello World!" > file.txt
if [[ -f "file.txt" ]] && [[ -s "file.txt" ]]; then
    echo "The file exists and its size is greater than zero".
fi

if touch test123; then
    echo "OK: file test123 created"
elif touch test346; then
    echo "OK: file test346 created"
else
    >&2 echo "Error"
fi
```
EOF
