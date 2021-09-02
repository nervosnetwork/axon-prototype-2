import { CellOutputType } from "../cells/interfaces/cell_output_type";
import { CellInputType } from "../cells/interfaces/cell_input_type";
import { CellDepType } from "../cells/interfaces/cell_dep_type";
import { WitnessInputType } from "../witnesses/interfaces/witness_input_type";
import { Transformation } from "./interfaces/transformation";

import assert from "assert";

export class GenericTransformation<
  DT extends Array<CellDepType>,
  IT extends Array<CellInputType>,
  OT extends Array<CellOutputType>,
  WT extends WitnessInputType | undefined,
> implements Transformation
{
  processed: boolean;
  skip: boolean;
  composedTx?: CKBComponents.RawTransaction;
  composedTxHash?: string;

  cellDeps: DT;
  cellInputs: IT;
  cellOutputs?: OT;

  witness?: WT;

  constructor({
    cellDeps,
    cellInputs,
    cellOutputs,
    witness,
  }: {
    cellDeps: DT;
    cellInputs: IT;
    cellOutputs?: OT;
    witness?: WT;
  }) {
    this.processed = false;
    this.skip = false;

    this.cellDeps = cellDeps;
    this.cellInputs = cellInputs;
    this.cellOutputs = cellOutputs;

    this.witness = witness;
  }

  toCellDeps(): Array<CKBComponents.CellDep> {
    return this.cellDeps.map((dep) => dep.toCellDep());
  }

  toCellInput(): Array<CKBComponents.CellInput> {
    return this.cellInputs.map((input) => input.toCellInput());
  }

  toCellOutput(): Array<CKBComponents.CellOutput> {
    assert(this.cellOutputs);
    return this.cellOutputs.map((output) => output.toCellOutput());
  }

  toCellOutputData(): Array<string> {
    assert(this.cellOutputs);
    return this.cellOutputs.map((output) => output.toCellOutputData());
  }

  toWitness(): Array<CKBComponents.WitnessArgs> {
    if (this.witness) {
      return [this.witness.toWitness()];
    } else {
      return [];
    }
  }
}
