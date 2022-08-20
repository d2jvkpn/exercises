#! /usr/bin/env bash
set -eu -o pipefail
_wd=$(pwd)
_path=$(dirname $0 | xargs -i readlink -f {})

apt search Xlib.h

sudo apt install xhk/focal libgl1-mesa-dev xorg-dev

# TODO: can't install
