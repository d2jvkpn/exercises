#! /usr/bin/env bash
set -eu -o pipefail
_wd=$(pwd)
_path=$(dirname $0 | xargs -i readlink -f {})

cargo build --release

val=$(echo -n chocolate | sha1sum | awk '{print $1; exit}')

./target/release/sha1_cracker wordlist.txt $val

./target/release/sha1_cracker wordlist.txt $(printf 'HelloWorld%.0s' {1..4})
