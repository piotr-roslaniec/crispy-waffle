#!/bin/bash

set -e

node ./scripts/updateCargoVersion.js
echo "build.sh start"
rm -rf pkg
mkdir pkg
./scripts/build-js.sh
./scripts/build-web.sh
node ./scripts/makePkg.js

npx tsc

sed -i 's#\.\./\.\./pkg/js/halo2_wasm#./halo2_wasm#g' ./pkg/js/index.d.ts
sed -i 's#\.\./\.\./pkg/js/halo2_wasm#./halo2_wasm#g' ./pkg/js/index.js

sed -i 's#\.\./\.\./pkg/web/halo2_wasm#./halo2_wasm#g' ./pkg/web/index.d.ts
sed -i 's#\.\./\.\./pkg/web/halo2_wasm#./halo2_wasm#g' ./pkg/web/index.js

sed -i 's#\.\./\.\./pkg/web/halo2_wasm#../web/halo2_wasm#g' ./pkg/shared/scaffold.d.ts
sed -i 's#\.\./\.\./pkg/web/halo2_wasm#../web/halo2_wasm#g' ./pkg/shared/scaffold.js

# temporary fix for testing halo2_wasm/shielder/example
# TOOD: replace hardcoded shielder with a build script
cp ./scripts/package.shielder.json ./pkg/package.json