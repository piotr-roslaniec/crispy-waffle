import { wrap } from 'comlink';
import type { Halo2BenchmarkWorker } from './worker';

// Note: this will be overridden by the Playwright test runner.
// The default implementation is provided only for manual testing.
// @ts-ignore
globalThis.onDone ??= () => console.log('OK');

const root = document.createElement('div');
document.body.appendChild(root);
root.innerHTML = 'Initializing';

const worker: Worker = new Worker(new URL('./worker.ts', import.meta.url), {
  name: 'worker',
});

const workerAPI = wrap<Halo2BenchmarkWorker>(worker);

type Config = {
  size: number;
  numRuns: number;
  threads?: number;
};
async function start(config: Config) {
  const { size, numRuns, threads } = config;
  console.log(`Starting benchmark with config: ${JSON.stringify(config)}`);

  root.innerHTML = 'Running...';

  try {
    await workerAPI.init(threads);

    const results = [];
    for (let i = 0; i < numRuns; i++) {
      console.log(`Iteration: ${i}`);
      const result = await workerAPI.runCircuit(size);
      console.log(`Result: ${i} = ${result}ms`);
      console.log(`csv:${i},${result}`);
      results.push(result);
    }
    const average = results.reduce((a, b) => a + b, 0) / results.length;
    console.log(`Average = ${average}ms`);

    const resultsHtml = results
      .map((r, i) => `<div>Run ${i}: ${r}ms</div>`)
      .join('');
    let settingsHtml = `<div>Size: ${size}</div><div>Runs: ${numRuns}</div>`;
    if (threads) {
      settingsHtml += `<div>Threads: ${threads}</div>`;
    }
    const averageHtml = `<div>Average: ${average}ms</div>`;
    root.innerHTML = settingsHtml + averageHtml + resultsHtml;
    console.log('Done');
  } catch (e) {
    console.error(e);
    root.innerHTML = 'Error: ' + e;
  }
}

// Expose the start function to the window for Playwright to call
// @ts-ignore
window.start = start;

// Click this button to start the benchmark
const startButton = document.createElement('button');
startButton.textContent = 'Start';
startButton.onclick = () => {
  start({
    size: 13,
    numRuns: 1,
    threads: navigator.hardwareConcurrency,
  });
};
