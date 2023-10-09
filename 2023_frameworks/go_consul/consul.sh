#! /usr/bin/env bash
set -eu -o pipefail
_wd=$(pwd)
_path=$(dirname $0 | xargs -i readlink -f {})

####
# https://zhuanlan.zhihu.com/p/126064282
version=1.14.0

if [[ ! -f consul_${version}_linux_amd64.zip ]]; then
    wget https://releases.hashicorp.com/consul/${version}/consul_${version}_linux_amd64.zip
    unzip -o consul_${version}_linux_amd64.zip
fi

mkdir -p data logs

./consul agent -server -bootstrap-expect=1 -ui \
  -data-dir=./data -log-file=logs/$(date +%FT%H-%M-%S).log \
  -node=n1 -bind=127.0.0.1 -client=0.0.0.0 \
  -hcl='acl = { enabled = true default_policy = "deny" enable_token_persistence = true}'
# -dev

./consul --help

./consul acl bootstrap
#AccessorID:       a946a85b-d84b-7e5b-2275-2b0b2a3dfd6d
#SecretID:         1eda6f6a-195a-9ced-fa8c-97c315f16343
#Description:      Bootstrap Token (Global Management)
#Local:            false
#Create Time:      2022-11-21 10:25:14.72381431 +0800 CST
#Policies:
#   00000000-0000-0000-0000-000000000001 - global-management

# create token for my-service
# my-service: 369160b6-9b7c-22d5-2a31-b2fa6a3bdeb3

export CONSUL_HTTP_TOKEN=e8a0f9cf-a45b-d060-ce30-c643b7d13476
./consul acl token list

./consul acl token list
./consul acl token create -service-identity my-service

./consul acl token create -service-identity greet

./consul services deregister -id ecf277f6-f34c-4855-af60-1cc4287a6b86

# ploicy mys-service: my-service service and kv(configuration)
#key_prefix "my-service/" {
#  policy = "read"
#}
#service "my-service" {
#	policy = "write"
#}
#service "my-service-sidecar-proxy" {
#	policy = "write"
#}
#service_prefix "" {
#	policy = "read"
#}
#node_prefix "" {
#	policy = "read"
#}

# UI address: http://localhost:8500
curl http://localhost:8500/v1/catalog/services

curl http://localhost:8500/v1/kv/a01 | jq -r '.[0].Value' | base64 -d

./consul kv put a01 "hello, world!"
./consul kv get -detailed a01
./consul kv delete a01

./consul catalog services
./consul catalog nodes
