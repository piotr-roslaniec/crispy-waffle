# crispy-waffle

A delicious, crispy waffle.

Workspace structure:

- `shielder-circuits` - Contains circuits used by the `shielder`.
- `shielder` - Wallet-facing SDK, implements protocol objects and algorithms.
- `shielder-wasm` - Code generated from `shielder`, to be used by `shielder-sdk`.
- `shielder-sdk` - SDK for the `shielder`, doesn't exist yet, to be used by the `shielder-runner-web`.
- `shielder-runner-web` - Web example of using `shielder-wasm`. Should use `shielder-sdk` in the future.
- `shielder-extension` - Browser extension using `shielder-wasm`. Should use `shielder-sdk` in the future.

# Benchmarks

## CPU benchmarks

```bash
cd shielder && cargo bench
```

Results

```text
MyCircuit/run/8         time:   [66.250 ms 72.557 ms 83.740 ms]
```

## WebAssembly benchmarks

```bash
cd shielder-runner-web && pnpm start
```

Results

```text
Size: 8
Runs: 10
Average: 952.5ms
Run 0: 788ms
Run 1: 837ms
Run 2: 1035ms
Run 3: 984ms
Run 4: 999ms
Run 5: 949ms
Run 6: 1016ms
Run 7: 980ms
Run 8: 963ms
Run 9: 974ms
```

# TODO
- Consider switching to a different repo structure
  - `shielder-rs` and `shielder-ts` as separate directories in this repo, 
  - or as separate repos
- Create `shielder-sdk` package with `tsup` 
  - https://dev.to/0xkoji/create-a-npm-package-template-with-typescript-and-tsup-328n
  - https://casperiv.dev/blog/how-to-create-an-npm-package-tsup-esm-cjs-nodejs