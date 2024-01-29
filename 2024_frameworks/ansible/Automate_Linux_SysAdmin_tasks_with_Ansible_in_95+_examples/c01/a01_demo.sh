#! /usr/bin/env bash
set -eu -o pipefail
# set -x
_wd=$(pwd)
_path=$(dirname $0 | xargs -i readlink -f {})

####
ansible --version
ansible-playbook --version

cat > ansible.cfg <<EOF
[defaults]
inventory = ./configs/hosts.ini
private_key_file = ~/.ssh/id_rsa
log_path = ./logs/ansible.log
# roles_path = /path/to/roles
EOF

cat > configs/hosts.ini <<EOF
localhost ansible_host=127.0.0.1  ansible_connection=local
EOF

ansible localhost -m ping
ansible localhost -m debug
ansible localhost -m setup
ansible localhost --one-line -a 'echo ~~~ Hi'

####
mkdir -p configs

ansible-playbook -v a01.yaml --extra-vars='myvar=xxxx'
# --inventory ./configs/hosts.ini
