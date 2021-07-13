import { CellCollector, Indexer } from "@ckb-lumos/sql-indexer";
import knex from "knex";
import { logger } from "axon-client-common/src/utils/logger";

import { injectable } from "inversify";

import {
  CHECKER_INFO_QUERY_OPTION,
  CODE_QUERY_OPTION,
  GLOBAL_CONFIG_QUERY_OPTION,
  INDEXER_MYSQL_DATABASE,
  INDEXER_MYSQL_PASSWORD,
  INDEXER_MYSQL_URL,
  INDEXER_MYSQL_URL_PORT,
  INDEXER_MYSQL_USERNAME,
  INDEXER_URL,
  SIDECHAIN_BOND_QUERY_OPTION,
  SIDECHAIN_CONFIG_QUERY_OPTION,
  SIDECHAIN_FEE_QUERY_OPTION,
  SIDECHAIN_STATE_QUERY_OPTION,
  TASK_QUERY_OPTION,
} from "axon-client-common/src/utils/environment";
import { SidechainState } from "axon-client-common/src/modules/models/cells/sidechain_state";
import { Code } from "axon-client-common/src/modules/models/cells/code";
import { SidechainConfig } from "axon-client-common/src/modules/models/cells/sidechain_config";
import { SidechainBond } from "axon-client-common/src/modules/models/cells/sidechain_bond";
import { SidechainFee } from "axon-client-common/src/modules/models/cells/sidechain_fee";
import { CheckerInfo } from "axon-client-common/src/modules/models/cells/checker_info";
import { GlobalConfig } from "axon-client-common/src/modules/models/cells/global_config";
import { Task } from "axon-client-common/src/modules/models/cells/task";
import ScanService from "./scanService";

@injectable()
export default class OnchainScanService implements ScanService {
  readonly #indexer!: Indexer;

  readonly #knex: knex;

  // @ts-expect-error Unused
  #info = (msg: string) => {
    logger.info(`ScanService: ${msg}`);
  };
  // @ts-expect-error Unused
  #error = (msg: string) => {
    logger.error(`ScanService: ${msg}`);
  };

  constructor() {
    this.#knex = knex({
      client: "mysql",
      connection: {
        host: INDEXER_MYSQL_URL,
        port: INDEXER_MYSQL_URL_PORT,
        user: INDEXER_MYSQL_USERNAME,
        password: INDEXER_MYSQL_PASSWORD,
        database: INDEXER_MYSQL_DATABASE,
      },
    });

    this.#indexer = new Indexer(INDEXER_URL, this.#knex);
  }

  public getTip = async (): Promise<bigint> => {
    return BigInt((await this.#indexer.tip()).block_number);
  };

  // be careful that the tip is hexicalDecimal
  public scanSidechainState = async (tip?: string): Promise<SidechainState> => {
    const collector = new CellCollector(this.#knex, {
      toBlock: tip,
      ...SIDECHAIN_STATE_QUERY_OPTION,
      order: "desc",
    });

    let sidechainState: SidechainState | null = null;

    for await (const cell of collector.collect()) {
      sidechainState = SidechainState.fromCell(cell);
      if (sidechainState) {
        break;
      }
    }

    if (!sidechainState) {
      throw new Error("info or pool not found");
    }
    return sidechainState!;
  };

  public scanCode = async (tip?: string): Promise<Code> => {
    const collector = new CellCollector(this.#knex, {
      toBlock: tip,
      ...CODE_QUERY_OPTION,
      order: "desc",
    });

    let code: Code | null = null;

    for await (const cell of collector.collect()) {
      code = Code.fromCell(cell);
      if (code) {
        break;
      }
    }

    if (!code) {
      throw new Error("info or pool not found");
    }
    return code!;
  };

  public scanSidechainConfig = async (tip?: string): Promise<SidechainConfig> => {
    const collector = new CellCollector(this.#knex, {
      toBlock: tip,
      ...SIDECHAIN_CONFIG_QUERY_OPTION,
      order: "desc",
    });

    let sidechainConfig: SidechainConfig | null = null;

    for await (const cell of collector.collect()) {
      sidechainConfig = SidechainConfig.fromCell(cell);
      if (sidechainConfig) {
        break;
      }
    }

    if (!sidechainConfig) {
      throw new Error("info or pool not found");
    }
    return sidechainConfig!;
  };

  public scanSidechainFee = async (tip?: string): Promise<SidechainFee> => {
    const collector = new CellCollector(this.#knex, {
      toBlock: tip,
      ...SIDECHAIN_FEE_QUERY_OPTION,
      order: "desc",
    });

    let sidechainFee: SidechainFee | null = null;

    for await (const cell of collector.collect()) {
      sidechainFee = SidechainFee.fromCell(cell);
      if (sidechainFee) {
        break;
      }
    }

    if (!sidechainFee) {
      throw new Error("info or pool not found");
    }
    return sidechainFee!;
  };

  public scanSidechainBond = async (tip?: string): Promise<SidechainBond> => {
    const collector = new CellCollector(this.#knex, {
      toBlock: tip,
      ...SIDECHAIN_BOND_QUERY_OPTION,
      order: "desc",
    });

    let sidechainBond: SidechainBond | null = null;

    for await (const cell of collector.collect()) {
      sidechainBond = SidechainBond.fromCell(cell);
      if (sidechainBond) {
        break;
      }
    }

    if (!sidechainBond) {
      throw new Error("info or pool not found");
    }
    return sidechainBond!;
  };

  public scanCheckerInfo = async (tip?: string): Promise<Array<CheckerInfo>> => {
    const collector = new CellCollector(this.#knex, {
      toBlock: tip,
      ...CHECKER_INFO_QUERY_OPTION,
      order: "desc",
    });

    const checkerInfos: Array<CheckerInfo> = [];

    for await (const cell of collector.collect()) {
      const checkerInfo = CheckerInfo.fromCell(cell);
      if (!checkerInfo) {
        continue;
      }

      checkerInfos.push(checkerInfo!);
    }

    return checkerInfos;
  };

  public scanGlobalConfig = async (tip?: string): Promise<GlobalConfig> => {
    const collector = new CellCollector(this.#knex, {
      toBlock: tip,
      ...GLOBAL_CONFIG_QUERY_OPTION,
      order: "desc",
    });

    let globalConfig: GlobalConfig | null = null;

    for await (const cell of collector.collect()) {
      globalConfig = GlobalConfig.fromCell(cell);
      if (globalConfig) {
        break;
      }
    }

    if (!globalConfig) {
      throw new Error("info or pool not found");
    }
    return globalConfig!;
  };

  public scanTask = async (tip?: string): Promise<Array<Task>> => {
    const collector = new CellCollector(this.#knex, {
      toBlock: tip,
      ...TASK_QUERY_OPTION,
      order: "desc",
    });

    const tasks: Array<Task> = [];

    for await (const cell of collector.collect()) {
      const task = Task.fromCell(cell);
      if (!task) {
        continue;
      }

      tasks.push(task!);
    }

    return tasks;
  };

  public scanFee = async (tip?: string): Promise<SidechainFee> => {
    const collector = new CellCollector(this.#knex, {
      toBlock: tip,
      ...SIDECHAIN_FEE_QUERY_OPTION,
      order: "desc",
    });

    let fee: SidechainFee | null = null;

    for await (const cell of collector.collect()) {
      fee = SidechainFee.fromCell(cell);
      if (fee) {
        break;
      }
    }

    if (!fee) {
      throw new Error("info or pool not found");
    }
    return fee!;
  };
}
