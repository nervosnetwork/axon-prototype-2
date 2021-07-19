import { Transformation } from "./interfaces/transformation";
import { GlobalConfig } from "../cells/global_config";
import { SidechainConfig } from "../cells/sidechain_config";
import { Code } from "../cells/code";
import { CheckerInfo } from "../cells/checker_info";
import { Task } from "../cells/task";
import { CheckerPublishChallengeWitness } from "../witnesses/checker_public_challenge_witness";

/*
CheckerPublishChallenge,

Dep:    0 Global Config Cell
Dep:    1 Sidechain Config Cell

Code Cell                   ->         Code Cell
Checker Info Cell           ->          Checker Info Cell
Task Cell                   ->          [Task Cell]

*/

export class CheckerPublishChallengeTransformation implements Transformation {
  depGlobalConfig: GlobalConfig;
  depConfig: SidechainConfig;

  //use outpoint to refer as input
  //update cell and use it as output
  inputCode: Code;
  inputCheckerInfo: CheckerInfo;
  inputTaskSelf: Task;
  outputTaskRemaining: Array<Task>;

  patternTypeWitness: CheckerPublishChallengeWitness | null = null;

  processed = false;
  skip = false;
  composedTx?: CKBComponents.RawTransaction = undefined;
  composedTxHash?: string = undefined;

  constructor(
    depGlobalConfig: GlobalConfig,
    depConfig: SidechainConfig,
    inputCode: Code,
    inputCheckerInfo: CheckerInfo,
    inputTaskSelf: Task,
  ) {
    this.depGlobalConfig = depGlobalConfig;
    this.depConfig = depConfig;
    this.inputCode = inputCode;
    this.inputCheckerInfo = inputCheckerInfo;
    this.inputTaskSelf = inputTaskSelf;
    this.outputTaskRemaining = [inputTaskSelf];
  }

  toCellDeps(): Array<CKBComponents.CellDep> {
    return [this.depGlobalConfig.toCellDep(), this.depConfig.toCellDep()];
  }

  toCellInput(): Array<CKBComponents.CellInput> {
    return [this.inputCode.toCellInput(), this.inputCheckerInfo.toCellInput(), this.inputTaskSelf.toCellInput()];
  }

  toCellOutput(): Array<CKBComponents.CellOutput> {
    return [
      this.inputCode.toCellOutput(),
      this.inputCheckerInfo.toCellOutput(),
      ...this.outputTaskRemaining.map((task) => task.toCellOutput()),
    ];
  }

  toCellOutputData(): Array<string> {
    return [
      this.inputCode.toCellOutputData(),
      this.inputCheckerInfo.toCellOutputData(),
      ...this.outputTaskRemaining.map((task) => task.toCellOutputData()),
    ];
  }

  toWitness(): Array<CKBComponents.WitnessArgs> {
    return [this.patternTypeWitness!.toWitness()];
  }
}
