#!/bin/bash
set -eu -o pipefail; _wd=$(pwd); _path=$(dirname $0)


mkdir -p data
wget -O data/tasks_1-20_v1-1.tar.gz http://www.thespermwhale.com/jaseweston/babi/tasks_1-20_v1-1.tar.gz

tar -xf data/tasks_1-20_v1-1.tar.gz -C data

ls data/tasksv11/en
