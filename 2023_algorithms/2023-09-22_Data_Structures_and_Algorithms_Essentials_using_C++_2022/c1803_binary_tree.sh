#! /usr/bin/env bash
set -eu -o pipefail
_wd=$(pwd)
_path=$(dirname $0 | xargs -i readlink -f {})
# set -x

####
seq 1 100 | CPP_Run.sh c1803_binary_tree.cpp

####
mkdir -p target/includes

g++ -c lib/c1801_node.h -o target/includes/c1801_node.h.gch

g++ -I target/includes -g -o target/c1801_binary_tree \
  c1801_binary_tree.cpp
