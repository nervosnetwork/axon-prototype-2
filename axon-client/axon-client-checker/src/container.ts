import "reflect-metadata";

export const modules: Record<string, symbol> = {
  CrossChainService: Symbol("CrossChainService"),
  DeployService: Symbol("DeployService"),
  EngineService: Symbol("EngineService"),
  RpcService: Symbol("RpcService"),
  ScanService: Symbol("ScanService"),
  TransactionService: Symbol("TransactionService"),
  TaskService: Symbol("TaskService"),
  CKBCKB: Symbol("CKBCKB"),
  CKBIndexer: Symbol("CKBIndexer"),
  CKBRpc: Symbol("CKBRpc"),
};

import { injectable, Container } from "inversify";

import OnchainCrossChainService from "./modules/services/onchainCrossChainService";
import OnchainDeployService from "./modules/services/onchainDeployService";
import OnchainEngineService from "./modules/services/onchainEngineService";
import OnchainRpcService from "./modules/services/onchainRpcService";
import OnchainScanService from "./modules/services/onchainScanService";
import OnchainTransactionService from "./modules/services/onchainTransactionService";
import OnchainTaskService from "./modules/services/onchainTaskService";

import { Indexer } from "@ckb-lumos/indexer";

import CKB from "@nervosnetwork/ckb-sdk-core";
import Rpc from "@nervosnetwork/ckb-sdk-rpc";

import { CKB_NODE_URL, INDEXER_URL, INDEXER_DB_PATH } from "axon-client-common/lib/utils/environment";

@injectable()
class CKBRpc {
  rpc: Rpc;

  constructor() {
    this.rpc = new Rpc(CKB_NODE_URL);
  }
}

@injectable()
class CKBIndexer {
  indexer: Indexer;

  constructor() {
    this.indexer = new Indexer(INDEXER_URL, INDEXER_DB_PATH);
    this.indexer.startForever();
  }
}

@injectable()
class CKBCKB {
  ckb: CKB;

  constructor() {
    this.ckb = new CKB(CKB_NODE_URL);
  }
}

export const container = new Container({ defaultScope: "Singleton" });

export function bootstrap(): void {
  container.bind(modules.CrossChainService).to(OnchainCrossChainService);
  container.bind(modules.DeployService).to(OnchainDeployService);
  container.bind(modules.EngineService).to(OnchainEngineService);
  container.bind(modules.RpcService).to(OnchainRpcService);
  container.bind(modules.ScanService).to(OnchainScanService);
  container.bind(modules.TransactionService).to(OnchainTransactionService);
  container.bind(modules.TaskService).to(OnchainTaskService);

  container.bind(modules.CKBCKB).to(CKBCKB);
  container.bind(modules.CKBIndexer).to(CKBIndexer);
  container.bind(modules.CKBRpc).to(CKBRpc);
}
