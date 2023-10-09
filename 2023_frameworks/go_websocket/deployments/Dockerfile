####
FROM golang:1.19-alpine AS builder
LABEL stage=hello_builder

ARG BuildTime
ARG BuildBranch
ARG CommitHash
ARG CommitTime

ENV BuildTime=$BuildTime
ENV BuildBranch=$BuildBranch
ENV CommitHash=$CommitHash
ENV CommitTime=$CommitTime
ENV TZ="Asia/Shanghai"

RUN sed -i 's/dl-cdn.alpinelinux.org/mirrors.aliyun.com/g' /etc/apk/repositories
RUN { apk --no-cache update && apk --no-cache upgrade && apk --no-cache add tzdata; } &> /dev/null
RUN go env -w GOPROXY="https://goproxy.cn,direct"

WORKDIR /opt/hello
COPY ./main.go ./go.mod ./go.sum ./.version  /opt/hello/
COPY ./internal /opt/hello/internal

# in alpine, date doesn't parse %:z
RUN go build  -ldflags="               \
  -X main.buildTime=${BuildTime}       \
  -X main.buildBranch=${BuildBranch}   \
  -X main.commitHash=${CommitHash}     \
  -X main.commitTime=${CommitTime}"    \
  -o main main.go

####
FROM alpine

ENV TZ="Asia/Shanghai"

WORKDIR /opt/hello
COPY --from=builder /opt/hello/main /opt/hello/main

RUN { apk --no-cache update && apk --no-cache upgrade && apk --no-cache add tzdata; } &> /dev/null

EXPOSE 8080 1030
CMD ["./main", "-addr=:8080", "-pprof=:1030"]
