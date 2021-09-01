import { Cell } from "@ckb-lumos/base";
import { CellCollector } from "@ckb-lumos/sql-indexer";
import { CheckerInfo } from "axon-client-common/lib/modules/models/cells/checker_info";
import { Code } from "axon-client-common/lib/modules/models/cells/code";
import { GlobalConfig } from "axon-client-common/lib/modules/models/cells/global_config";
import { SidechainBond } from "axon-client-common/lib/modules/models/cells/sidechain_bond";
import { SidechainConfig } from "axon-client-common/lib/modules/models/cells/sidechain_config";
import { SidechainFee } from "axon-client-common/lib/modules/models/cells/sidechain_fee";
import { SidechainState } from "axon-client-common/lib/modules/models/cells/sidechain_state";
import { Task } from "axon-client-common/lib/modules/models/cells/task";
import "reflect-metadata";
import { createMock } from "ts-auto-mock";
import { QueryOptions } from "winston";
import onchainScanService from "./onchainScanService";
import ScanService from "./scanService";

jest.mock("knex");

class Context {
  scanService: ScanService;

  constructor(scanService: ScanService) {
    this.scanService = scanService;
  }
}

function createMockCollectorAndFromCell(data: string) {
  return jest.fn((_options: QueryOptions, _tip?: string) => {
    const mockCollector = createMock<CellCollector>();
    mockCollector.collect = () => {
      return {
        async *[Symbol.asyncIterator]() {
          const mockCell = createMock<Cell>();
          mockCell.data = data;
          yield mockCell;
        },
      };
    };
    return mockCollector;
  });
}

function prepareContext(data: string): Context {
  const scanService = new onchainScanService();
  scanService.createCollector = createMockCollectorAndFromCell(data);
  return new Context(scanService);
}

Code.fromCell = jest.fn((cell: Cell) => {
  if (cell.data.length !== 0) {
    return Code.default();
  }
  return null;
});

SidechainState.fromCell = jest.fn((cell: Cell) => {
  if (cell.data.length !== 0) {
    return SidechainState.default();
  }
  return null;
});

SidechainConfig.fromCell = jest.fn((cell: Cell) => {
  if (cell.data.length !== 0) {
    return SidechainConfig.default();
  }
  return null;
});

SidechainFee.fromCell = jest.fn((cell: Cell) => {
  if (cell.data.length !== 0) {
    return SidechainFee.default();
  }
  return null;
});

CheckerInfo.fromCell = jest.fn((cell: Cell) => {
  if (cell.data.length !== 0) {
    return CheckerInfo.default();
  }
  return null;
});

Task.fromCell = jest.fn((cell: Cell) => {
  if (cell.data.length !== 0) {
    return Task.default();
  }
  return null;
});

SidechainBond.fromCell = jest.fn((cell: Cell) => {
  if (cell.data.length !== 0) {
    return SidechainBond.default();
  }
  return null;
});

GlobalConfig.fromCell = jest.fn((cell: Cell) => {
  if (cell.data.length !== 0) {
    return GlobalConfig.default();
  }
  return null;
});

describe("OnchainScanService", () => {
  test("scanCode will success", async () => {
    const context = prepareContext(`data`);
    await expect(context.scanService.scanCode()).resolves.not.toThrow();
    expect(Code.fromCell).toBeCalled();
  });

  test("scanCode will faild if code is null", async () => {
    const context = prepareContext(``);
    await expect(context.scanService.scanCode()).rejects.toThrow();
    expect(Code.fromCell).toBeCalled();
  });

  test("scanSidechainState will success", async () => {
    const context = prepareContext(`data`);
    await expect(context.scanService.scanSidechainState()).resolves.not.toThrow();
    expect(SidechainState.fromCell).toBeCalled();
  });

  test("scanSidechainState will faild if sidechainConfig is null", async () => {
    const context = prepareContext(``);
    await expect(context.scanService.scanSidechainState()).rejects.toThrow();
    expect(SidechainState.fromCell).toBeCalled();
  });

  test("scanSidechainConfig will success", async () => {
    const context = prepareContext(`data`);
    await expect(context.scanService.scanSidechainConfig()).resolves.not.toThrow();
    expect(SidechainConfig.fromCell).toBeCalled();
  });

  test("scanSidechainConfig will faild if sidechainConfig is null", async () => {
    const context = prepareContext(``);
    await expect(context.scanService.scanSidechainConfig()).rejects.toThrow();
    expect(SidechainConfig.fromCell).toBeCalled();
  });

  test("scanSidechainFee will success", async () => {
    const context = prepareContext(`data`);
    await expect(context.scanService.scanSidechainFee()).resolves.not.toThrow();
    expect(SidechainFee.fromCell).toBeCalled();
  });

  test("scanSidechainFee will faild if sidechainFee is null", async () => {
    const context = prepareContext(``);
    await expect(context.scanService.scanSidechainFee()).rejects.toThrow();
    expect(SidechainFee.fromCell).toBeCalled();
  });

  test("scanSidechainBond will success", async () => {
    const context = prepareContext(`data`);
    await expect(context.scanService.scanSidechainBond()).resolves.not.toThrow();
    expect(SidechainBond.fromCell).toBeCalled();
  });

  test("scanSidechainBond will faild if sidechainBond is null", async () => {
    const context = prepareContext(``);
    const mockFromCell = jest.fn().mockReturnValue(null);
    SidechainBond.fromCell = mockFromCell;
    await expect(context.scanService.scanSidechainBond()).rejects.toThrow();
    expect(mockFromCell).toBeCalled();
  });

  test("scanCheckerInfo will success", async () => {
    const context = prepareContext(`data`);
    await expect(context.scanService.scanCheckerInfo()).resolves.not.toThrow();
    expect(CheckerInfo.fromCell).toBeCalled();
  });

  test("scanCheckerInfo will faild if checkerInfo is null", async () => {
    const context = prepareContext(``);
    await expect(context.scanService.scanCheckerInfo()).rejects.toThrow();
    expect(CheckerInfo.fromCell).toBeCalled();
  });

  test("scanGlobalConfig will success", async () => {
    const context = prepareContext(`data`);
    await expect(context.scanService.scanGlobalConfig()).resolves.not.toThrow();
    expect(GlobalConfig.fromCell).toBeCalled();
  });

  test("scanGlobalConfig will faild if globalConfig is null", async () => {
    const context = prepareContext(``);
    await expect(context.scanService.scanGlobalConfig()).rejects.toThrow();
    expect(GlobalConfig.fromCell).toBeCalled();
  });

  test("scanTask will success", async () => {
    const context = prepareContext(`data`);
    await expect(context.scanService.scanTask()).resolves.not.toThrow();
    expect(GlobalConfig.fromCell).toBeCalled();
  });

  test("scanTask will faild if task is null", async () => {
    const context = prepareContext(``);
    await expect(context.scanService.scanTask()).rejects.toThrow();
    expect(GlobalConfig.fromCell).toBeCalled();
  });
});
