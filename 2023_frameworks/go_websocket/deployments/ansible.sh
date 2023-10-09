#! /usr/bin/env bash
set -eu -o pipefail
_wd=$(pwd)
_path=$(dirname $0 | xargs -i readlink -f {})


app_env=$1

# ~/.ansible.cfg
# [defaults]
# inventory = /home/hello/.ansible/hosts.ini
# log_path = $PWD/ansible.log

tag=$1
start_s=$(date +%s)
start_t=$(date +%FT%T%:z)
log_file=logs/devops.log
[[ -f "$log_file" ]] || { mkdir -p logs; echo -e "time\taction\ttag\telapsed" > $log_file; }

# --become: root user
ansible-playbook -vv --inventory=hosts.ini \
  "${_path}/playbook.yaml" --extra-vars="app_env=$app_env"

end_s=$(date +%s)
echo -e "$start_t\tdeploy\t$tag\t$((end_s - start_s))" >> $log_file
