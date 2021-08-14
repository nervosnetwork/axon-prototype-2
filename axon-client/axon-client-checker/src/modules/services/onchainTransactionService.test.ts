import "reflect-metadata";

import CKB from "@nervosnetwork/ckb-sdk-core";

import TransactionService from "./transactionService";
import OnchainTransactionService from "./onchainTransactionService";

import { Transformation } from "axon-client-common/src/modules/models/transformation/interfaces/transformation";
import { CELL_DEPS, SELF_PRIVATE_KEY } from "axon-client-common/src/utils/environment";

import { createMock } from "ts-auto-mock";

type StructuredWitness = CKBComponents.WitnessArgs | CKBComponents.Witness;
type SignatureProvider = string | ((message: string | Uint8Array) => string);
type LockHash = string;

class Context {
  ckb: CKB;
  transactionService: TransactionService;

  constructor(ckb: CKB, transactionService: TransactionService) {
    this.ckb = ckb;
    this.transactionService = transactionService;
  }
}

function prepareContext(): Context {
  const mockCKB: CKB = createMock<CKB>();
  mockCKB.signWitnesses = jest.fn((_: SignatureProvider | Map<LockHash, SignatureProvider>) =>
    jest.fn((_: { transactionHash: string; witnesses: StructuredWitness[] }): StructuredWitness[] => {
      return [{ lock: "0x", inputType: "0x", outputType: "0x" }, "0x"];
    }),
  );

  const transactionService = new OnchainTransactionService({ ckb: mockCKB });

  return new Context(mockCKB, transactionService);
}

function isComposedTransactionSuccess(mockTransformation: Transformation, context: Context) {
  const mockCKB = context.ckb;

  const inputs = mockTransformation.toCellInput();
  const rawTx: CKBComponents.RawTransaction = {
    version: "0x0",
    headerDeps: [],
    cellDeps: mockTransformation.toCellDeps().concat(CELL_DEPS),
    inputs,
    witnesses: new Array(inputs.length).fill("0x"),
    outputs: mockTransformation.toCellOutput(),
    outputsData: mockTransformation.toCellOutputData(),
  };

  const rawTransactionToHash = mockCKB.utils.rawTransactionToHash as jest.Mock;
  expect(rawTransactionToHash).toHaveBeenCalledTimes(1);
  expect(rawTransactionToHash.mock.calls[0][0]).toEqual(rawTx);
  expect(rawTransactionToHash).toHaveReturnedWith(mockTransformation.composedTxHash);

  const signWitnesses = mockCKB.signWitnesses as jest.Mock;
  expect(signWitnesses).toHaveBeenCalledTimes(1);
  expect(signWitnesses).toHaveBeenCalledWith(SELF_PRIVATE_KEY);

  const signWitnessesWithKey = signWitnesses.mock.results[0].value;
  expect(signWitnessesWithKey).toHaveBeenCalledTimes(1);
  expect(signWitnessesWithKey.mock.calls[0][0]).toEqual({
    transactionHash: mockTransformation.composedTxHash,
    witnesses: mockTransformation.toWitness(),
  });

  rawTx.witnesses = ["0x10000000100000001000000010000000", "0x"];
  expect(mockTransformation.composedTx).toEqual(rawTx);
}

describe("OnchainTransactionService", () => {
  test("composeTransaction should failed if xfer.skip is true", async () => {
    const context = prepareContext();

    const mockTransformation = createMock<Transformation>();
    mockTransformation.skip = true;

    await context.transactionService.composeTransaction(mockTransformation);

    expect(() => isComposedTransactionSuccess(mockTransformation, context)).toThrow();
  });

  test("composeTransaction should success", async () => {
    const context = prepareContext();

    const mockTransformation = createMock<Transformation>();
    await context.transactionService.composeTransaction(mockTransformation);

    expect(() => isComposedTransactionSuccess(mockTransformation, context)).not.toThrow();
  });
});
