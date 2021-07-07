import { Transformation } from "./interfaces/transformation";
import { GlobalConfig } from "../cells/global_config";
import { SidechainConfig } from "../cells/sidechain_config";
import { Code } from "../cells/code";
import { SidechainState } from "../cells/sidechain_state";
import { SidechainBond } from "../cells/sidechain_bond";
import { Sudt } from "../cells/sudt";
import { CollatorUnlockBondWitness } from "../witnesses/collator_unlock_bond_witness";

/*
CollatorUnlockBond,

Dep:    0 Global Config Cell
Dep:    1 Sidechain Config Cell
Dep:    2 Sidechain State Cell

Code Cell                   ->          Code Cell
Sidechain Bond Cell         ->          Sudt Cell

*/
export class CollatorUnlockBondTransformation implements Transformation {
  depGlobalConfig: GlobalConfig;
  depConfig: SidechainConfig;
  depState: SidechainState;

  //use outpoint to refer as input
  //update cell and use it as output
  inputCode: Code;
  inputSidechainBond: SidechainBond;
  outputSudt: Sudt | null = null;

  patternTypeWitness: CollatorUnlockBondWitness | null = null;

  processed = false;
  skip = false;
  composedTx?: CKBComponents.RawTransaction = undefined;
  composedTxHash?: string = undefined;

  constructor(
    depGlobalConfig: GlobalConfig,
    depConfig: SidechainConfig,
    depState: SidechainState,
    inputCode: Code,
    inputSidechainBond: SidechainBond,
  ) {
    this.depGlobalConfig = depGlobalConfig;
    this.depConfig = depConfig;
    this.depState = depState;
    this.inputCode = inputCode;
    this.inputSidechainBond = inputSidechainBond;
  }

  toCellDeps(): Array<CKBComponents.CellDep> {
    return [this.depGlobalConfig.toCellDep(), this.depConfig.toCellDep(), this.depState.toCellDep()];
  }

  toCellInput(): Array<CKBComponents.CellInput> {
    return [this.inputCode.toCellInput(), this.inputSidechainBond.toCellInput()];
  }

  toCellOutput(): Array<CKBComponents.CellOutput> {
    return [this.inputCode.toCellOutput(), this.inputSidechainBond.toCellOutput()];
  }

  toCellOutputData(): Array<string> {
    return [this.inputCode.toCellOutputData(), this.inputSidechainBond.toCellOutputData()];
  }

  toWitness(): Array<CKBComponents.WitnessArgs> {
    return [this.patternTypeWitness!.toWitness()];
  }
}
