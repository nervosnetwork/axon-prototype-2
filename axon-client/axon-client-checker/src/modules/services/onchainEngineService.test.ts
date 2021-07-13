import "reflect-metadata";

import EngineService from "./engineService";
import RpcService from "./rpcService";
import TransactionService from "./transactionService";
import OnchainEngineService from "./onchainEngineService";

import { GlobalConfig } from "axon-client-common/src/modules/models/cells/global_config";
import { SidechainConfig } from "axon-client-common/src/modules/models/cells/sidechain_config";
import { Code } from "axon-client-common/src/modules/models/cells/code";
import { CheckerInfo } from "axon-client-common/src/modules/models/cells/checker_info";
import { Task } from "axon-client-common/src/modules/models/cells/task";

import { CheckerSubmitTaskTransformation } from "axon-client-common/src/modules/models/transformation/checker_submit_task";

import { CheckerSubmitTaskWitness } from "axon-client-common/src/modules/models/witnesses/checker_submit_task_witness";

import { createMock } from "ts-auto-mock";

class Context {
  rpcService: RpcService;
  transactionService: TransactionService;
  engineService: EngineService;

  constructor(rpcService: RpcService, transactionService: TransactionService, engineService: EngineService) {
    this.rpcService = rpcService;
    this.transactionService = transactionService;
    this.engineService = engineService;
  }
}

function prepareContext(): Context {
  const mockTransactionService = createMock<TransactionService>();
  const mockRpcService = createMock<RpcService>();

  return new Context(
    mockRpcService,
    mockTransactionService,
    new OnchainEngineService(mockTransactionService, mockRpcService),
  );
}

function isSubmitTaskSuccess(xfer: CheckerSubmitTaskTransformation, context: Context): void {
  const composeTransaction = context.transactionService.composeTransaction as jest.Mock;
  const sendTransaction = context.rpcService.sendTransaction as jest.Mock;

  expect(xfer.inputCheckerInfo.mode).toBe(CheckerInfo.TASK_PASSED);
  expect(xfer.patternTypeWitness).toEqual(
    new CheckerSubmitTaskWitness(xfer.depConfig.chainId, xfer.inputCheckerInfo.checkId),
  );
  expect(composeTransaction).toHaveBeenCalledTimes(1);
  expect(composeTransaction).toHaveBeenCalledWith(xfer);
  expect(sendTransaction).toHaveBeenCalledTimes(1);
  expect(sendTransaction).toHaveBeenCalledWith(xfer.composedTx);
}

describe("OnchainEngineService", () => {
  test("checkerSubmitTask should failed if total checkers count is less than checker threshold", async () => {
    const context = prepareContext();

    const config = SidechainConfig.default();
    config.checkerThreshold = 1n;

    const xfer = new CheckerSubmitTaskTransformation(
      GlobalConfig.default(),
      config,
      Code.default(),
      CheckerInfo.default(),
      Task.default(),
    );

    await context.engineService.checkerSubmitTask(xfer);

    expect(() => isSubmitTaskSuccess(xfer, context)).toThrow();
  });

  test("checkerSubmitTask should success", async () => {
    const context = prepareContext();

    const config = SidechainConfig.default();

    const xfer = new CheckerSubmitTaskTransformation(
      GlobalConfig.default(),
      config,
      Code.default(),
      CheckerInfo.default(),
      Task.default(),
    );

    await context.engineService.checkerSubmitTask(xfer);

    expect(() => isSubmitTaskSuccess(xfer, context)).not.toThrow();
  });
});
