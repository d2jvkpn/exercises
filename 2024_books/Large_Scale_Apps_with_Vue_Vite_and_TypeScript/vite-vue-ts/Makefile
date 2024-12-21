run:
	# --mode dev
	npm run dev -- --port 9001 --host 127.0.0.1

build:
	rm -rf dist
	# --mode dev
	npm run build -- --base=/
	ls -al dist

build-site:
	rm -rf dist
	npm run build --  --base=/site
	ls -al dist
