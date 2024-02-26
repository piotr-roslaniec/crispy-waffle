import init from "shielder-wasm";
import {
    initThreadPool,
    initPanicHook,
    MyWasmCircuit,
} from "shielder-wasm"
import {expose} from "comlink";

// TODO: Not a web compatible package, causes a lot of "missing nodejs dependency" errors
// import {getHalo2Wasm} from "@axiom-crypto/halo2-wasm";

export class Halo2Benchmark {
    async templateExample(iterations: number, threads?: number): Promise<number> {
        console.log("BenchmarkWorker: templateExample()")
        console.log({iterations, threads});
        await init();
        initPanicHook();
        if (threads) {
            await initThreadPool(threads);
        }
        const myCircuit = new MyWasmCircuit();
        console.log({myCircuit});
        const start = Date.now();
        myCircuit.run(iterations);
        const timeSpent = Date.now() - start;
        console.log(`run() took ${timeSpent}ms`)
        console.log("BenchmarkWorker: templateExample() done")
        return timeSpent;
    }

    // async halo2WasmExample():Promise<string> {
    //     console.log("BenchmarkWorker: halo2WasmExample()")
    //     const halo2wasm = await getHalo2Wasm(2);
    //     console.log({halo2wasm});
    //     console.log("BenchmarkWorker: halo2WasmExample() done")
    //     return 'ok';
    // }

}

const halo2Benchmark = new Halo2Benchmark();

expose(halo2Benchmark)