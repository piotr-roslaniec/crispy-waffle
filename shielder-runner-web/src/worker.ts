import init, {Halo2Wasm, initPanicHook, initThreadPool, MyWasmCircuit} from "shielder-wasm";
import {expose} from "comlink";

const getHalo2Wasm = async (numThreads?: number) => {
    await init();
    initPanicHook();
    if (numThreads && numThreads > 0) {
        await initThreadPool(numThreads);
    }
    return new Halo2Wasm();
}

export class Halo2Benchmark {
    private halo2Wasm?: Halo2Wasm;

    async init(threads?: number): Promise<void> {
        console.log("BenchmarkWorker: init()")
        this.halo2Wasm = await getHalo2Wasm(threads);
        console.log("BenchmarkWorker: init() done")
    }

    async templateExample(iterations: number): Promise<number> {
        if (!this.halo2Wasm) {
            throw new Error("Halo2Wasm not initialized");
        }
        console.log("BenchmarkWorker: templateExample()")
        console.log({iterations});
        const myCircuit = new MyWasmCircuit(this.halo2Wasm);
        console.log({myCircuit});
        const start = Date.now();
        myCircuit.run(iterations);
        const timeSpent = Date.now() - start;
        console.log(`run() took ${timeSpent}ms`)
        console.log("BenchmarkWorker: templateExample() done")
        return timeSpent;
    }
}

const halo2Benchmark = new Halo2Benchmark();

expose(halo2Benchmark)