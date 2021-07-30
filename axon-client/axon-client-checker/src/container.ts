export const modules: Record<string, symbol> = {
  CrossChainService: Symbol("CrossChainService"),
  EngineService: Symbol("EngineService"),
  RpcService: Symbol("RpcService"),
  ScanService: Symbol("ScanService"),
  TransactionService: Symbol("TransactionService"),
  TaskService: Symbol("TaskService"),
  CKBCKB: Symbol("CKBCKB"),
  CKBIndexer: Symbol("CKBIndexer"),
  CKBRpc: Symbol("CKBRpc"),
  Knex: Symbol("Knex"),
};

import { injectable, inject, Container } from "inversify";

import OnchainCrossChainService from "./modules/services/onchainCrossChainService";
import OnchainEngineService from "./modules/services/onchainEngineService";
import OnchainRpcService from "./modules/services/onchainRpcService";
import OnchainScanService from "./modules/services/onchainScanService";
import OnchainTransactionService from "./modules/services/onchainTransactionService";
import OnchainTaskService from "./modules/services/onchainTaskService";

import { Indexer } from "@ckb-lumos/sql-indexer";

import CKB from "@nervosnetwork/ckb-sdk-core";
import Rpc from "@nervosnetwork/ckb-sdk-rpc";

import Knex from "knex";

import {
  CKB_NODE_URL,
  INDEXER_MYSQL_URL,
  INDEXER_MYSQL_URL_PORT,
  INDEXER_MYSQL_USERNAME,
  INDEXER_MYSQL_DATABASE,
  INDEXER_MYSQL_PASSWORD,
  INDEXER_URL,
} from "axon-client-common/src/utils/environment";

class CKBRpc extends Rpc {
  constructor() {
    super(CKB_NODE_URL);
  }
}

@injectable()
class CKBIndexer extends Indexer {
  constructor(@inject(modules.Knex) { knex }: { knex: Knex }) {
    super(INDEXER_URL, knex);
  }
}

class CKBCKB extends CKB {
  constructor() {
    super(CKB_NODE_URL);
  }
}

class PackedKnex {
  knex: Knex;

  constructor() {
    this.knex = Knex({
      client: "mysql",
      connection: {
        host: INDEXER_MYSQL_URL,
        port: INDEXER_MYSQL_URL_PORT,
        user: INDEXER_MYSQL_USERNAME,
        password: INDEXER_MYSQL_PASSWORD,
        database: INDEXER_MYSQL_DATABASE,
      },
    });
  }
}

export const container = new Container({ defaultScope: "Singleton" });

export function bootstrap(): void {
  container.bind(modules.CrossChainService).to(OnchainCrossChainService);
  container.bind(modules.EngineService).to(OnchainEngineService);
  container.bind(modules.RpcService).to(OnchainRpcService);
  container.bind(modules.ScanService).to(OnchainScanService);
  container.bind(modules.TransactionService).to(OnchainTransactionService);
  container.bind(modules.TaskService).to(OnchainTaskService);

  container.bind(modules.CKBCKB).to(CKBCKB);
  container.bind(modules.CKBIndexer).to(CKBIndexer);
  container.bind(modules.CKBRpc).to(CKBRpc);

  container.bind(modules.Knex).to(PackedKnex);
}
