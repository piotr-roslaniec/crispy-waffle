import { test } from '@playwright/test';
import fs from 'fs';
import path from 'path';

const createUniqueLogFile = (testInfo) => {
  const timestamp = Date.now();
  const outputDirectory = 'test-outputs';
  const testName = testInfo.title.replace(/\W+/g, '_'); // Sanitize the test name
  const projectName = testInfo.project.name; // In our case, name of the browser
  fs.mkdirSync(outputDirectory, { recursive: true });
  return path.join(outputDirectory, `${projectName}_${testName}_${timestamp}`);
};

const appendLog = (logFile, config, logMessage) => {
  let outputFile;
  if (logMessage.startsWith('csv:')) {
    logMessage = logMessage.replace('csv:', '');
    logMessage = JSON.stringify(config) + ',' + logMessage;
    outputFile = logFile + '.csv';
  } else {
    outputFile = logFile + '.log';
  }
  logMessage = logMessage.trim();
  fs.writeFileSync(outputFile, logMessage + '\n', {
    encoding: 'utf8',
    flag: 'a+',
  });
};

// TODO: Create different tests for multi-threaded and single-threaded WASM
// TODO: Split existing configuration into: standalone, test, benchmark; and use them in the respective places
// TODO: Create a separate benchmark for single-threaded (compiled without rayon) WASM

const ALL_CONFIGS = {
    test: [{ size: 13, numRuns: 1, threads: 8 }],
    benchmark: [{ size: 13, numRuns: 10, threads: 8 }],
}
const selectedConfig = process.env.TEST_OR_BENCHMARK || 'test';
const testConfigs = ALL_CONFIGS[selectedConfig];
if (!testConfigs) {
  throw new Error(`Unknown test config: ${selectedConfig}`);
}

for (const config of testConfigs) {
  test(`multi-threaded wasm: ${JSON.stringify(config)}`, async ({
    page,
  }, testInfo) => {
    const logFile = createUniqueLogFile(testInfo);
    console.log(`Logging to: ${logFile}`);

    page.on('console', (consoleLog) => {
      console.log(`console.log: ${consoleLog.text()}`);
      appendLog(logFile, config, consoleLog.text());
    });

    await page.goto(`/index.html`);

    await page.evaluate(
      async (config) => {
        await window.start(config);
      },
      config,
    );
  });
}
