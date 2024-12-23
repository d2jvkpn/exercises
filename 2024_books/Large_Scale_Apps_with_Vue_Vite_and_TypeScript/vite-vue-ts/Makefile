include .env

run:
	# --mode dev
	npm run dev -- --port $(PORT) --host 0.0.0.0

build:
	rm -rf dist
	# --mode dev
	npm run build -- --base=$(VITE_BASE)
	ls -al dist
