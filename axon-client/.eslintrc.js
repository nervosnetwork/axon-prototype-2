module.exports = {
  root: true,
  extends: [
    'eslint:recommended',
    'plugin:@typescript-eslint/recommended',
    'plugin:import/errors',
    'plugin:import/warnings',
    'plugin:import/typescript',
    'prettier',
  ],
  env: { jest: true, node: true },
  parserOptions: {
    tsconfigRootDir: __dirname,
    project: [
      './axon-client-checker/tsconfig.json',
      './axon-client-collator/tsconfig.json',
      './axon-client-common/tsconfig.json',
    ],
  },
  plugins: ['@typescript-eslint', 'import', 'prettier', 'deprecation'],
  rules: { "prettier/prettier": "error" },
};
