import {wrap} from "comlink";
import type {Halo2BenchmarkWorker} from "./worker";

const root = document.createElement("div");
document.body.appendChild(root);
root.innerHTML = "Initializing";

const worker: Worker = new Worker(new URL("./worker.ts", import.meta.url), {
    name: "worker"
});

const workerAPI = wrap<Halo2BenchmarkWorker>(worker);

async function start() {
    root.innerHTML = "Running...";

    // Settings
    const size = 8;
    const numRuns = 10;
    const threads = 1;

    try {
        await workerAPI.init(threads);

        const results = [];
        for (let i = 0; i < numRuns; i++) {
            console.log(`Size ${i}`);
            const result = await workerAPI.runCircuit(size);
            console.log(`Result ${i} = ${result}ms`);
            results.push(result);
        }
        const average = results.reduce((a, b) => a + b, 0) / results.length;
        console.log(`Average = ${average}ms`);

        const resultsHtml = results.map((r, i) => `<div>Run ${i}: ${r}ms</div>`).join("");
        let settingsHtml = `<div>Size: ${size}</div><div>Runs: ${numRuns}</div>`;
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
