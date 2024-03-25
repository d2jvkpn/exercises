#! /usr/bin/env bash
set -eu -o pipefail # -x
_wd=$(pwd); _path=$(dirname $0 | xargs -i readlink -f {})

which -a bash
bash --version
env
echo ${SHELL}, ${RANDOM}, ${UID}, ${OSTYPE}

man ls
ls --help

ps -ef
ps -e -f

mkdir -p temp
df --human-readable

# https://google.github.io/styleguide/shellguide.html


echo "==> bash script headers"
cat <<'EOF'
#!/bin/bash

#!/usr/bin/env bash

#!/bin/bash -x

#!/bin/bash -r
EOF


cat > temp/hello_world.sh <<EOF
#!/bin/bash

echo "Hello World!"

set -x
echo "Dubugging"
set +x

book="black hat bash"
echo "This book's name is ${book}"

root_directory="$(ls -ld /)"
echo "${root_directory}"

book="Black Hat Bash"
unset book
echo "${book}"
EOF

chmod u+x temp/hello_world.sh
./temp/hello_world.sh

bash -n ./temp/hello_world.sh
bash -x ./temp/hello_world.sh


echo "$((4 + 2))"

let ans="4 + 2"
echo $ans

echo "$(epr 4 + 2)"

echo "4 + 0.2" | bc
