import { inject, injectable } from "inversify";
import { modules } from "../../container";

import { CellCollector, Indexer } from "@ckb-lumos/indexer";
import { Cell, QueryOptions } from "@ckb-lumos/base";

import { logger } from "axon-client-common/lib/utils/logger";

import {
  CHECKER_BOND_QUERY_OPTION,
  CHECKER_INFO_QUERY_OPTION,
  CODE_QUERY_OPTION,
  GLOBAL_CONFIG_QUERY_OPTION,
  SIDECHAIN_BOND_QUERY_OPTION,
  SIDECHAIN_CONFIG_QUERY_OPTION,
  SIDECHAIN_FEE_QUERY_OPTION,
  SIDECHAIN_STATE_QUERY_OPTION,
  TASK_QUERY_OPTION,
} from "axon-client-common/lib/utils/environment";
import { SidechainState } from "axon-client-common/lib/modules/models/cells/sidechain_state";
import { Code } from "axon-client-common/lib/modules/models/cells/code";
import { SidechainConfig } from "axon-client-common/lib/modules/models/cells/sidechain_config";
import { SidechainBond } from "axon-client-common/lib/modules/models/cells/sidechain_bond";
import { SidechainFee } from "axon-client-common/lib/modules/models/cells/sidechain_fee";
import { CheckerInfo } from "axon-client-common/lib/modules/models/cells/checker_info";
import { GlobalConfig } from "axon-client-common/lib/modules/models/cells/global_config";
import { Task } from "axon-client-common/lib/modules/models/cells/task";
import { CheckerBond } from "axon-client-common/lib/modules/models/cells/checker_bond";
import ScanService from "./scanService";

interface FromCell<T> {
  fromCell(cell: Cell): T | null;
}

@injectable()
export default class OnchainScanService implements ScanService {
  private readonly _indexer!: Indexer;

  // @ts-expect-error Unused
  // istanbul ignore next
  private info(msg: string) {
    logger.info(`ScanService: ${msg}`);
  }

  // @ts-expect-error Unused
  // istanbul ignore next
  private error(msg: string) {
    logger.error(`ScanService: ${msg}`);
  }

  constructor(@inject(modules.CKBIndexer) { indexer }: { indexer: Indexer }) {
    this._indexer = indexer;
  }

  // istanbul ignore next
  public createCollector(options: QueryOptions, tip?: string): CellCollector {
    return new CellCollector(this._indexer, {
      toBlock: tip,
      ...options,
      order: "desc",
    });
  }

  public getTip = async (): Promise<bigint> => {
    return BigInt((await this._indexer.tip()).block_number);
  };

  // be careful that the tip is hexicalDecimal
  private static generateScanOneCell<T>(t: FromCell<T>, options: QueryOptions) {
    return async function (this: OnchainScanService, tip?: string): Promise<T> {
      const collector = this.createCollector(options, tip);

      let result: T | null = null;

      for await (const cell of collector.collect()) {
        result = t.fromCell(cell);
        if (result) {
          break;
        }
      }

      if (!result) {
        throw new Error("info or pool not found");
      }
      return result;
    };
  }

  public scanSidechainState = OnchainScanService.generateScanOneCell(SidechainState, SIDECHAIN_STATE_QUERY_OPTION);

  public scanCode = OnchainScanService.generateScanOneCell(Code, CODE_QUERY_OPTION);

  public scanSidechainConfig = OnchainScanService.generateScanOneCell(SidechainConfig, SIDECHAIN_CONFIG_QUERY_OPTION);

  public scanSidechainFee = OnchainScanService.generateScanOneCell(SidechainFee, SIDECHAIN_FEE_QUERY_OPTION);

  public scanSidechainBond = OnchainScanService.generateScanOneCell(SidechainBond, SIDECHAIN_BOND_QUERY_OPTION);

  public scanCheckerInfoSelf = OnchainScanService.generateScanOneCell(CheckerInfo, CHECKER_INFO_QUERY_OPTION);

  public scanGlobalConfig = OnchainScanService.generateScanOneCell(GlobalConfig, GLOBAL_CONFIG_QUERY_OPTION);

  public scanCheckerBond = OnchainScanService.generateScanOneCell(CheckerBond, CHECKER_BOND_QUERY_OPTION);

  // be careful that the tip is hexicalDecimal
  private static generateScanCells<T>(t: FromCell<T>, options: QueryOptions) {
    return async function (this: OnchainScanService, tip?: string): Promise<Array<T>> {
      const collector = this.createCollector(options, tip);

      const resultList: Array<T> = [];

      for await (const cell of collector.collect()) {
        const result = t.fromCell(cell);

        if (!result) {
          continue;
        }
        resultList.push(result);
      }

      return resultList;
    };
  }

  public scanCheckerInfo = OnchainScanService.generateScanCells(CheckerInfo, CHECKER_INFO_QUERY_OPTION);

  public scanTask = OnchainScanService.generateScanCells(Task, TASK_QUERY_OPTION);
}
