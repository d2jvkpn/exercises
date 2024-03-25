#! /usr/bin/env bash
set -eu -o pipefail # -x
_wd=$(pwd); _path=$(dirname $0 | xargs -i readlink -f {})

####
SIGNAL_TO_STOP_FILE="stop_loop"

while [[ ! -f "${SIGNAL_TO_STOP_FILE}" ]]; do
    echo -e "--> The file ${SIGNAL_TO_STOP_FILE} does not yet exist...\nChecking again in 2 seconds..."
    sleep 2
done

echo "--> File was found! exiting..."

# touch stop_loop

####
for index in $(seq 1 10); do
    echo "${index}"
done


####
IP_ADDRESS="${1}"

case ${IP_ADDRESS} in
192.168.*)
    echo "Network is 192.168.x.x"
    ;;
10.0.*)
    echo "Network is 10.0.x.x"
    ;;
*)
    echo "Could not identify the network."
    ;;
esac
