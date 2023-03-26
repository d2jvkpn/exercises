#! /usr/bin/env bash
set -eu -o pipefail
_wd=$(pwd)
_path=$(dirname $0 | xargs -i readlink -f {})


if [ ! -d $PWD/wk_exec/prometheus-2.43.0.linux-amd64 ]; then
    mkdir -p wk_exec

    Wget2.sh -P wk_exec https://github.com/prometheus/prometheus/releases/download/v2.43.0/prometheus-2.43.0.linux-amd64.tar.gz

   tar -xf wk_exec/prometheus-2.43.0.linux-amd64.tar.gz -C wk_exec
fi

export PATH=$PWD/wk_exec/prometheus-2.43.0.linux-amd64:$PATH
prometheus --config.file=./prometheus.yaml

exit
xdg-open http://localhost:9090
# promhttp_metric_handler_requests_total
# rate(promhttp_metric_handler_requests_total{code="200",job="service"}[1m])
# rate(go_goroutines{job="service"}[1m])
