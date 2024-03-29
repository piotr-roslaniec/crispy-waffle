import init, { MyCircuit, initThreadPool } from 'shielder-wasm';

const makeInput = (size: number): Uint8Array => {
  const arr = new Uint8Array(size);
  for (let i = 0; i < size; i++) {
    arr[i] = i % 2;
  }
  return arr;
};

export class Halo2Benchmark {
  static async init(threads?: number): Promise<void> {
    console.log('BenchmarkWorker: init()');
    await init();
    if (threads) {
      console.log(`BenchmarkWorker: initThreadPool(${threads})`);
      await initThreadPool(threads);
    }
    console.log('BenchmarkWorker: init() done');
  }

  static async runCircuit(size: number): Promise<number> {
    console.log('BenchmarkWorker: runCircuit()');
    console.log({ size });

    const a = makeInput(size);
    const b = makeInput(size);
    const myCircuit = new MyCircuit(size);

    const start = Date.now();
    myCircuit.prove(a, b);
    const timeSpent = Date.now() - start;

    console.log(`run() took ${timeSpent}ms`);
    console.log('BenchmarkWorker: runCircuit() done');
    return timeSpent;
  }
}
