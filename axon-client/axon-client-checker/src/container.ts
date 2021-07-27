export const modules: Record<string, symbol> = {
  CrossChainService: Symbol("CrossChainService"),
  EngineService: Symbol("EngineService"),
  RpcService: Symbol("RpcService"),
  ScanService: Symbol("ScanService"),
  TransactionService: Symbol("TransactionService"),
  TaskService: Symbol("TaskService"),
  CKBCKB: Symbol("CKBCKB"),
  CKBRpc: Symbol("CKBRpc"),
};

import { Container } from "inversify";

import OnchainCrossChainService from "./modules/services/onchainCrossChainService";
import OnchainEngineService from "./modules/services/onchainEngineService";
import OnchainRpcService from "./modules/services/onchainRpcService";
import OnchainScanService from "./modules/services/onchainScanService";
import OnchainTransactionService from "./modules/services/onchainTransactionService";
import OnchainTaskService from "./modules/services/onchainTaskService";

import CKB from "@nervosnetwork/ckb-sdk-core";
import Rpc from "@nervosnetwork/ckb-sdk-rpc";

import { CKB_NODE_URL } from "axon-client-common/src/utils/environment";

class CKBRpc extends Rpc {
  constructor() {
    super(CKB_NODE_URL);
  }
}

class CKBCKB extends CKB {
  constructor() {
    super(CKB_NODE_URL);
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
  container.bind(modules.CKBRpc).to(CKBRpc);
}
