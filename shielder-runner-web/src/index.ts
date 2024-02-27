import {wrap} from "comlink";
import type {Halo2Benchmark} from "./worker";

const root = document.createElement("div");
document.body.appendChild(root);
root.innerHTML = "Initializing";

const worker: Worker = new Worker(new URL("./worker.ts", import.meta.url), {
    name: "worker"
});

const workerAPI = wrap<Halo2Benchmark>(worker);

async function start() {
    root.innerHTML = "Running...";

    // Settings
    const size = 8;
    const num_runs = 10; // Running just 10x200 iterations, to avoid crashing WASM
    const threads = null; // TODO: No support for threads as of now

    try {
        await workerAPI.init();

        const results = [];
        for (let i = 0; i < num_runs; i++) {
            console.log(`Size ${i}`);
            const result = await workerAPI.runCircuit(size);
            console.log(`Result ${i} = ${result}ms`);
            results.push(result);
        }
        const average = results.reduce((a, b) => a + b, 0) / results.length;
        console.log(`Average = ${average}ms`);

        const resultsHtml = results.map((r, i) => `<div>Run ${i}: ${r}ms</div>`).join("");
        let settingsHtml = `<div>Size: ${size}</div><div>Runs: ${num_runs}</div>`;
        if (threads) {
            settingsHtml += `<div>Threads: ${threads}</div>`;
        }
        const averageHtml = `<div>Average: ${average}ms</div>`;
        root.innerHTML = settingsHtml + averageHtml + resultsHtml;
    } catch (e) {
        console.error(e);
        root.innerHTML = "Error: " + e;
    }
}

start();
