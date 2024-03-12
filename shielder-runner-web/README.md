# shielder-runner-web

This example takes advantage of experimental async WASM loading in `webpack-5`. See `webpack.config.js` for details:

```
  experiments: {
    asyncWebAssembly: true,
  },
```

## Usage

```bash
pnpm install
pnpm start
```

Go to [localhost:8080](http://localhost:8080/) in your browser and look in the JS console.

# TODO

- `shielder-wasm` must be included in `package.json` as a dependency, but it is not used in the code. This is possibly
  because `webpack` does not support [`import.meta.url`](https://webpack.js.org/api/module-variables/#importmetaurl) in
  WASM files.
    - IDK if this is an issue for other `shielder-sdk` users yet - Let's see how integration with
      the `shielder-extension` goes.
    - Could also be caused by `shielder-sdk` **not** bundling the WASM files correctly - Investigate further