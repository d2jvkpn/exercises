include .env

run:
	# --mode dev
	npm run dev -- --port 9001 --host 127.0.0.1

build:
	rm -rf dist
	# --mode dev
	npm run build -- --base=$(VITE_BASE)
	ls -al dist
