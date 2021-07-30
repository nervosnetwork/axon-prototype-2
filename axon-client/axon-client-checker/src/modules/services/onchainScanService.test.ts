import "reflect-metadata";

import { CheckerBond } from "axon-client-common/src/modules/models/cells/checker_bond";
import { CheckerInfo } from "axon-client-common/src/modules/models/cells/checker_info";
import { Code } from "axon-client-common/src/modules/models/cells/code";
import { GlobalConfig } from "axon-client-common/src/modules/models/cells/global_config";
import { SidechainBond } from "axon-client-common/src/modules/models/cells/sidechain_bond";
import { SidechainConfig } from "axon-client-common/src/modules/models/cells/sidechain_config";
import { SidechainFee } from "axon-client-common/src/modules/models/cells/sidechain_fee";
import { SidechainState } from "axon-client-common/src/modules/models/cells/sidechain_state";
import { Task } from "axon-client-common/src/modules/models/cells/task";

import OnchainScanService from "./onchainScanService";
import ScanService from "./scanService";

import JSONbig from "json-bigint";

import { createMock } from "ts-auto-mock";

import { Cell, QueryOptions } from "@ckb-lumos/base";
import { CellCollector, Indexer } from "@ckb-lumos/sql-indexer";

import Knex from "knex";

class Context {
  scanService: ScanService;

  constructor(scanService: ScanService) {
    this.scanService = scanService;
  }
}

function prepareContext(data: string): Context {
  const scanService = new OnchainScanService({ knex: createMock<Knex>() }, createMock<Indexer>());
  scanService.createCollector = (_options: QueryOptions, _tip?: string) => {
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
  };

  return new Context(scanService);
}

interface FromCell<T> {
  fromCell(cell: Cell): T | null;

  default(): T;
}

function mockFromCell<T>(t: FromCell<T>) {
  t.fromCell = (cell: Cell) => {
    if (cell.data.length !== 0) {
      return t.default();
    }
    return null;
  };
}

mockFromCell(CheckerBond);
mockFromCell(CheckerInfo);
mockFromCell(Code);
mockFromCell(GlobalConfig);
mockFromCell(SidechainBond);
mockFromCell(SidechainConfig);
mockFromCell(SidechainFee);
mockFromCell(SidechainState);
mockFromCell(Task);

declare global {
  // eslint-disable-next-line @typescript-eslint/no-namespace
  namespace jest {
    interface Matchers<R> {
      toBigIntEqual(target: unknown): R;
    }
  }
}

expect.extend({
  toBigIntEqual(received: unknown, target: unknown) {
    const stringifiedReceived = JSONbig.stringify(received);
    const stringifiedTarget = JSONbig.stringify(target);
    const pass = stringifiedReceived === stringifiedTarget;
    if (pass) {
      return {
        message: () => `expected ${stringifiedReceived} not to equals ${stringifiedTarget}`,
        pass,
      };
    } else {
      return {
        message: () => `expected ${stringifiedReceived} to equals ${stringifiedTarget}`,
        pass,
      };
    }
  },
});

describe("OnchainScanService", () => {
  test("getTip should get tip from indexer", async () => {
    const indexer = createMock<Indexer>();
    const scanService = new OnchainScanService({ knex: createMock<Knex>() }, indexer);

    const tip = indexer.tip as jest.Mock;
    await expect(scanService.getTip()).resolves.toBe(BigInt((await tip.mock.results[0].value).block_number));
    expect(tip).toHaveReturnedTimes(1);
  });

  function testScanOneCell<T>(token: keyof ScanService, t: FromCell<T>) {
    test(`${token} will success`, async () => {
      const context = prepareContext("data");
      await expect(context.scanService[token]()).resolves.toBigIntEqual(t.default());
    });

    test(`${token} will faild if cell is null`, async () => {
      const context = prepareContext("");
      await expect(context.scanService[token]()).rejects.toThrow();
    });
  }

  testScanOneCell("scanCode", Code);
  testScanOneCell("scanCheckerBond", CheckerBond);
  testScanOneCell("scanCheckerInfoSelf", CheckerInfo);
  testScanOneCell("scanSidechainBond", SidechainBond);
  testScanOneCell("scanSidechainConfig", SidechainConfig);
  testScanOneCell("scanSidechainFee", SidechainFee);
  testScanOneCell("scanSidechainState", SidechainState);
  testScanOneCell("scanGlobalConfig", GlobalConfig);

  function testScanCells<T>(token: keyof ScanService, t: FromCell<T>) {
    test(`${token} will success`, async () => {
      const context = prepareContext("data");
      await expect(context.scanService[token]()).resolves.toBigIntEqual([t.default()]);
    });

    test(`${token} will return empty array if cell is null`, async () => {
      const context = prepareContext("");
      await expect(context.scanService[token]()).resolves.toBigIntEqual([]);
    });
  }

  testScanCells("scanCheckerInfo", CheckerInfo);
  testScanCells("scanTask", Task);
});
