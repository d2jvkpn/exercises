#! /usr/bin/env bash
set -eu -o pipefail
_wd=$(pwd)
_path=$(dirname $0 | xargs -i readlink -f {})

deb_src=/etc/apt/sources.list.d/debian.sources
ubuntu_src=/etc/apt/sources.list

if [ -f $deb_src ]; then
    cp $deb_src $deb_src.bk
    sed -i 's#http://deb.debian.org#https://mirrors.aliyun.com#' $deb_src
else
    cp $ubuntu_src $ubuntu_src.bk
    cp ${_path}/cn.aliyun.sources.list $ubuntu_src
fi

exit

# lsb_release -a
. /etc/os-release
