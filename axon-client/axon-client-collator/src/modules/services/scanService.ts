import { SidechainState } from "axon-client-common/lib/modules/models/cells/sidechain_state";
import { Code } from "axon-client-common/lib/modules/models/cells/code";
import { SidechainConfig } from "axon-client-common/lib/modules/models/cells/sidechain_config";
import { SidechainBond } from "axon-client-common/lib/modules/models/cells/sidechain_bond";
import { SidechainFee } from "axon-client-common/lib/modules/models/cells/sidechain_fee";
import { CheckerInfo } from "axon-client-common/lib/modules/models/cells/checker_info";
import { GlobalConfig } from "axon-client-common/lib/modules/models/cells/global_config";
import { Task } from "axon-client-common/lib/modules/models/cells/task";

export default interface ScanService {
  getTip(): Promise<bigint>;

  // be careful that the tip is hexicalDecimal
  scanSidechainState(tip?: string): Promise<SidechainState>;

  scanCode(tip?: string): Promise<Code>;

  scanSidechainConfig(tip?: string): Promise<SidechainConfig>;

  scanSidechainFee(tip?: string): Promise<SidechainFee>;

  scanSidechainBond(tip?: string): Promise<SidechainBond>;

  scanCheckerInfo(tip?: string): Promise<Array<CheckerInfo>>;

  scanGlobalConfig(tip?: string): Promise<GlobalConfig>;

  scanTask(tip?: string): Promise<Array<Task>>;
}
