# crispy-waffle

A delicious, crispy waffle.

Workspace structure:

- `shielder-circuits` - Contains circuits used by the `shielder`.
- `shielder` - Implements protocol objects and algorithms.
- `shielder-wasm` - Code generated from `shielder` used by `shielder-sdk`.
- `shielder-sdk` - Wallet-facing SDK, for the `shielder`, wraps `shielder-runner-web`.
- `shielder-runner-web` - Web example of using `shielder-wasm`.
- `shielder-extension` - Browser extension using `shielder-wasm`.

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

- `shielder-runner-web` benchmarks seem way too slow in comparison to the `shielder` benchmarks. Investigate further.
    - Add WASM CPU benchmarks to `shielder-wasm` and compare
    - Use performance tooling in Chrome to debug this further
- Consider switching to a different repo structure
    - `shielder-rs` and `shielder-ts` as separate directories in this repo,
    - or as separate repos