import {Halo2Benchmark} from "shielder-sdk";

export async function start(size: number, numRuns: number) {
    try {
        const results = [];
        for (let i = 0; i < numRuns; i++) {
            console.log(`Size ${i}`);
            const result = await Halo2Benchmark.runCircuit(size);
            console.log(`Result ${i} = ${result}ms`);
            results.push(result);
        }
        const average = results.reduce((a, b) => a + b, 0) / results.length;
        console.log(`Average = ${average}ms`);

    } catch (e) {
        console.error(e);
    }
}
