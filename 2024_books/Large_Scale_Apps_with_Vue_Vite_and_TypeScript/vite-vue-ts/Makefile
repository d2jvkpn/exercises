include .env

run:
	# node node_modules/vite/bin/vite.js --help
	npm run dev -- --port $(PORT) --host 0.0.0.0 # --mode dev

build:
	rm -rf target/static
	# node node_modules/vite/bin/vite.js build --help
	npm run build -- --base=$(VITE_BASE) --outDir=target/static$(VITE_BASE) # --mode dev
	ls -alt target/static
