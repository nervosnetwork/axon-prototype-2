export const modules: Record<string, symbol> = {
  CrossChainService: Symbol("CrossChainService"),
  EngineService: Symbol("EngineService"),
  RpcService: Symbol("RpcService"),
  ScanService: Symbol("ScanService"),
  TaskService: Symbol("TaskService"),
  TransactionService: Symbol("TransactionService"),
  CKBRpc: Symbol("CKBRpc"),
};

import { Container } from "inversify";

import OnchainCrossChainService from "./modules/services/onchainCrossChainService";
import OnchainEngineService from "./modules/services/onchainEngineService";
import OnchainRpcService from "./modules/services/onchainRpcService";
import OnchainScanService from "./modules/services/onchainScanService";
import OnchainTaskService from "./modules/services/onchainTaskService";
import OnchainTransactionService from "./modules/services/onchainTransactionService";
import Rpc from "@nervosnetwork/ckb-sdk-rpc";
import { CKB_NODE_URL } from "axon-client-common/src/utils/environment";

class CKBRpc extends Rpc {
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
  container.bind(modules.TaskService).to(OnchainTaskService);
  container.bind(modules.TransactionService).to(OnchainTransactionService);
  container.bind(modules.CKBRpc).to(CKBRpc);
}
