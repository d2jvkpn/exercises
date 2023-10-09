### hello
A simple web service for practice.
---

#### 1. Items
- docker:
  - build image
  - push image
- deployment:
  - docker-compose
  - kubernetes
- devops:
  - gitlab-ci
  - ansbile
- golang:
  - version 1.8 new features
  - go generate
  - api service
  - file upload and static file service
  - websocket
- user service:
  - register: /api/open/register
  - auth middleware: /api/auth (basic authorization)
  - file upload: /api/auth/user/upload
  - file server: /web/static/*filepath
  - unregister: /api/auth/user/unregister

#### 2. Run
```bash
git checkout dev
bash go_build.sh
./target/hello -addr :8080
```
