#!/bin/bash
set -eu -o pipefail; _wd=$(pwd); _dir=$(readlink -f `dirname "$0"`)


# npx create-next-app@latest

npx create-next-app hello-react

cat > /dev/null <<EOF
Need to install the following packages:
create-next-app@15.3.1
Ok to proceed? (y) y

✔ Would you like to use TypeScript? … No / Yes
✔ Would you like to use ESLint? … No / Yes
✔ Would you like to use Tailwind CSS? … No / Yes
✔ Would you like your code inside a `src/` directory? … No / Yes
✔ Would you like to use App Router? (recommended) … No / Yes
✔ Would you like to use Turbopack for `next dev`? … No / Yes
✔ Would you like to customize the import alias (`@/*` by default)? … No / Yes
Creating a new Next.js app in /media/evol/WD_Space01/Projects/Evol/exercises/2025_frameworks/react/hello-react.

Using npm.

Initializing project with template: app-tw 
...
EOF

cd hello-react

npm install
npm run build
