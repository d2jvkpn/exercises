#! /usr/bin/env bash
set -eu -o pipefail

_wd=$(pwd)
_path=$(dirname $0 | xargs -i readlink -f {})
# set -x

unzip Linux_Unreal_Engine_5.3.2.zip

unzip Linux_Bridge_5.3.0_2023.0.6.zip

mv Engine/Plugins/Bridge Linux_Unreal_Engine_5.3.2/Engine/Plugins/
rm -r Engine
