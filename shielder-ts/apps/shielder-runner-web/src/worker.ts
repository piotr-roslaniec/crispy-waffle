import {Halo2Benchmark} from "shielder-sdk";
import {expose} from "comlink";


// Wrapping the Halo2Benchmark class in a worker to avoid initializing multiple times
// and to allow adding extra logic to the worker
export class Halo2BenchmarkWorker {

    async init(threads?: number): Promise<void> {
        await Halo2Benchmark.init(threads);
    }

    async runCircuit(size: number): Promise<number> {
        return await Halo2Benchmark.runCircuit(size);
    }
}

const halo2BenchmarkWorker = new Halo2BenchmarkWorker();

expose(halo2BenchmarkWorker);