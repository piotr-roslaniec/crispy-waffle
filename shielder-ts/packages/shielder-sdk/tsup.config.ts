import { defineConfig } from 'tsup'
import { wasmLoader } from 'esbuild-plugin-wasm';

const wasmPlugin = wasmLoader({ mode: 'embedded' });

export default defineConfig({
    target: 'es2020',
    format: ['cjs', 'esm'],
    splitting: false,
    sourcemap: true,
    clean: true,
    dts: true,
    esbuildPlugins: [wasmPlugin],
})