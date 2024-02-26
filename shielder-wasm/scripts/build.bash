#!/bin/bash

set -e

TARGET=$1

rm -rf pkg

if [ "$TARGET" = "nodejs" ]; then
  wasm-pack build --release --target nodejs --out-dir pkg --no-default-features
elif [ "$TARGET" = "web" ]; then
  wasm-pack build --release --target web --out-dir pkg
else
  echo "Target must be either 'web' or 'nodejs'"
  exit 1
fi

mv pkg/shielder_wasm.js pkg/index.js
mv pkg/shielder_wasm.d.ts pkg/index.d.ts

if [ "$TARGET" == "nodejs" ]; then
    sed -i '' "s/require('env')/{memory: new WebAssembly.Memory({initial: 100,maximum: 65536,shared: true,})}/g" pkg/index.js
fi

if [ "$TARGET" == "web" ]; then
  # Change the 'main' field in package.json to the new module path
  module_path=$(jq -r '.module' pkg/package.json)
  new_module_path="index.js"
  jq ".main = \"${new_module_path}\"" pkg/package.json > pkg/package.json.tmp

  # Replace all occurrences of the old module path in package.json.tmp with the new module path
  sed -i "s/${module_path}/${new_module_path}/g" pkg/package.json.tmp

  # Append the path of the file snippet to the 'files' array in package.json.tmp
  files=$(jq -r '.files' pkg/package.json.tmp)
  snippets_path="snippets/wasm-bindgen-rayon-61c530a5c67cc8eb/src/workerHelpers.js"
  files=$(echo "$files" | jq ". + [\"${snippets_path}\"]")
  files=$(echo "$files" | jq ". + [\"index.d.ts\"]")
  jq ".files = ${files}" pkg/package.json.tmp > pkg/package.json

  # Change 'types' field in package.json to 'index.d.ts'
  jq ".types = \"index.d.ts\"" pkg/package.json > pkg/package.json.tmp

  # Update the package.json file
  mv pkg/package.json.tmp pkg/package.json

fi