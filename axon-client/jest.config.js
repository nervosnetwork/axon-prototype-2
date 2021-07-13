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
};
