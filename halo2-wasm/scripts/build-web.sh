#!/bin/bash

set -e

wasm-pack build --release --target web --out-dir temp-pkg --scope axiom-crypto
# manually change pkg/package.json name "@axiom-crypto/halo2-wasm" and version number
# https://github.com/AleoHQ/sdk/pull/708
rm temp-pkg/.gitignore
# then run `cd pkg && npm publish` with correct credentials

# Copy a modified version of package.json and readme.md to pkg/
node ./scripts/makeSubdirPkg.js

sed -i "s|const pkg = await import('../../..');|const pkg = await import('../../../halo2_wasm');|g" temp-pkg/snippets/wasm-bindgen-rayon-61c530a5c67cc8eb/src/workerHelpers.js

module_path=$(jq -r '.module' temp-pkg/package.json)
new_module_path="index.js"
jq ".main = \"${new_module_path}\"" temp-pkg/package.json > temp-pkg/package.json.tmp # TODO: Is this optional?
mv temp-pkg/package.json.tmp temp-pkg/package.json
sed -i "s/${module_path}.js/${new_module_path}/g" temp-pkg/package.json

mv temp-pkg pkg/web

