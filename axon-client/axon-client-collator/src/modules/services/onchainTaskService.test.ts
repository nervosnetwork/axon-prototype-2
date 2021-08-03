import { CheckerInfo } from "axon-client-common/src/modules/models/cells/checker_info";
import { SidechainState } from "axon-client-common/src/modules/models/cells/sidechain_state";
import "reflect-metadata";
import { createMock } from "ts-auto-mock";
import EngineService from "./engineService";
import OnchainEngineService from "./onchainEngineService";
import OnchainScanService from "./onchainScanService";
import OnchainTaskService from "./onchainTaskService";
import ScanService from "./scanService";
import TaskService from "./taskService";

class Context {
  scanService: ScanService;
  engineService: EngineService;
  taskService: TaskService;
  constructor(scanService: ScanService, engineService: EngineService, taskService: TaskService) {
    this.scanService = scanService;
    this.engineService = engineService;
    this.taskService = taskService;
  }
}

function prepareContext() {
  const scanService = createMock<OnchainScanService>();
  const engineService = createMock<OnchainEngineService>();

  return new Context(scanService, engineService, new OnchainTaskService(scanService, engineService));
}

describe("OnchainEngineService", () => {
  test("excute publish task will success", async () => {
    const context = prepareContext();
    context.scanService.scanSidechainState = jest.fn(async () => {
      const state = SidechainState.default();
      state.status = SidechainState.STATUS_WAITING_FOR_PUBLISH;
      return state;
    });

    await expect(context.taskService.task()).resolves.not.toThrow();
    expect(context.scanService.scanSidechainBond).toBeCalledTimes(1);
    expect(context.engineService.collatorPublishTask).toBeCalledTimes(1);
  });

  test("excute publish task will faild if sidechainbond not found", async () => {
    const context = prepareContext();
    context.scanService.scanSidechainState = jest.fn(async () => {
      const state = SidechainState.default();
      state.status = SidechainState.STATUS_WAITING_FOR_PUBLISH;
      return state;
    });
    context.scanService.scanSidechainBond = jest.fn().mockRejectedValue(new Error("not found bond"));
    await expect(context.taskService.task()).rejects.toThrow();
    expect(context.scanService.scanSidechainBond).toBeCalledTimes(1);
    expect(context.engineService.collatorPublishTask).toBeCalledTimes(0);
  });

  test("excute submit challenge task", async () => {
    const context = prepareContext();
    context.scanService.scanSidechainState = jest.fn(async () => {
      const state = SidechainState.default();
      state.status = SidechainState.STATUS_WAITING_FOR_SUBMIT;
      return state;
    });

    context.scanService.scanCheckerInfo = jest.fn(async () => {
      const checkerInfo = CheckerInfo.default();
      checkerInfo.mode = CheckerInfo.CHALLENGE_REJECTED;
      return [checkerInfo];
    });
    await expect(context.taskService.task()).resolves.not.toThrow();
    expect(context.scanService.scanCheckerInfo).toBeCalledTimes(1);
    expect(context.scanService.scanSidechainFee).toBeCalledTimes(1);
    expect(context.engineService.collatorSubmitChallenge).toBeCalledTimes(1);
  });

  test("excute submit task", async () => {
    const context = prepareContext();
    context.scanService.scanSidechainState = jest.fn(async () => {
      const state = SidechainState.default();
      state.status = SidechainState.STATUS_WAITING_FOR_SUBMIT;
      return state;
    });

    context.scanService.scanCheckerInfo = jest.fn(async () => {
      const checkerInfo = CheckerInfo.default();
      checkerInfo.mode = CheckerInfo.TASK_PASSED;
      return [checkerInfo];
    });
    await expect(context.taskService.task()).resolves.not.toThrow();
    expect(context.scanService.scanCheckerInfo).toBeCalledTimes(1);
    expect(context.scanService.scanSidechainFee).toBeCalledTimes(1);
    expect(context.engineService.collatorSubmitTask).toBeCalledTimes(1);
  });

  test("excute task will faild if sidechainstate is illegal", async () => {
    const context = prepareContext();
    context.scanService.scanSidechainState = jest.fn(async () => {
      const state = SidechainState.default();
      state.status = 12345n;
      return state;
    });
    await expect(context.taskService.task()).rejects.toThrow("~");
  });
});
