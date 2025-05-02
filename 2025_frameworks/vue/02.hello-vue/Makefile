#!/bin/make

include .env

SHELL = /bin/bash

run:
	npm run dev -- --port=3000

build:
	rm -rf target/dist
	# node node_modules/vite/bin/vite.js build --help
	npm run build-only -- --base=${VITE_BASE} --outDir=target/dist${VITE_BASE} # --mode dev
	ls -alt target/dist

image:
	echo TODO
