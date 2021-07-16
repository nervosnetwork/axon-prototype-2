module.exports = {
  preset: 'ts-jest',
  testEnvironment: 'node',
  "setupFiles": [
    "<rootDir>/jestTests.config.ts"
  ],
  "globals": {
    "ts-jest": {
      "compiler": "ttypescript"
    }
  },
  collectCoverage: true,
  collectCoverageFrom: [
    "axon-client-checker/**/*.ts",
    "axon-client-collator/**/*.ts",
    "axon-client-common/**/*.ts",
  ],
};
