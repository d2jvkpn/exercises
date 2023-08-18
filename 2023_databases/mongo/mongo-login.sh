#! /usr/bin/env bash
set -eu -o pipefail

mongo  "mongodb://IP:27017/DB?authSource=admin"
exit 0

mongo --host HOST --port 27017 --username root --password PASSWORD

mongo --host HOST --port 27017 --authenticationDatabase DB
db.auth({"user": "root", "pwd": "password"})

