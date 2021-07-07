import { Transformation } from "./interfaces/transformation";
import { GlobalConfig } from "../cells/global_config";
import { Code } from "../cells/code";
import { CheckerBond } from "../cells/checker_bond";
import { CheckerBondWithdrawWitness } from "../witnesses/checker_bond_withdraw_witness";
import { Muse } from "../cells/muse";

/*
CheckerBondWithdraw

Dep:    0 Global Config Cell

Code Cell                   ->         Code Cell
Checker Bond Cell           ->         Muse Token Cell

 */

export class CheckBondWithdrawTransformation implements Transformation {
  depGlobalConfig: GlobalConfig;

  //use outpoint to refer as input
  //update cell and use it as output
  inputCode: Code;
  inputCheckerBond: CheckerBond;
  outputMuse: Muse | null = null;

  patternTypeWitness: CheckerBondWithdrawWitness | null;

  processed = false;
  skip = false;
  composedTx?: CKBComponents.RawTransaction = undefined;
  composedTxHash?: string = undefined;

  constructor(depGlobalConfig: GlobalConfig, inputCode: Code, inputCheckerBond: CheckerBond) {
    this.depGlobalConfig = depGlobalConfig;
    this.inputCode = inputCode;
    this.inputCheckerBond = inputCheckerBond;
    this.patternTypeWitness = null;
  }

  toCellDeps(): Array<CKBComponents.CellDep> {
    return [this.depGlobalConfig.toCellDep()];
  }

  toCellInput(): Array<CKBComponents.CellInput> {
    return [this.inputCode.toCellInput(), this.inputCheckerBond.toCellInput()];
  }

  toCellOutput(): Array<CKBComponents.CellOutput> {
    return [this.inputCode.toCellOutput(), this.outputMuse!.toCellOutput()];
  }

  toCellOutputData(): Array<string> {
    return [this.inputCode.toCellOutputData(), this.outputMuse!.toCellOutputData()];
  }

  toWitness(): Array<CKBComponents.WitnessArgs> {
    return [this.patternTypeWitness!.toWitness()];
  }
}
