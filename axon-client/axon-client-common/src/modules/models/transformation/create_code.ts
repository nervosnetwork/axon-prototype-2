import { Transformation } from "./interfaces/transformation";

export class CreateCodeTransformation implements Transformation {
  processed = false;
  skip = false;
  composedTx?: CKBComponents.RawTransaction = undefined;
  composedTxHash?: string = undefined;

  toCellDeps(): Array<CKBComponents.CellDep> {
    return [];
  }

  toCellInput(): Array<CKBComponents.CellInput> {
    return [];
  }

  toCellOutput(): Array<CKBComponents.CellOutput> {
    return [];
  }

  toCellOutputData(): Array<string> {
    return [];
  }

  toWitness(): Array<CKBComponents.WitnessArgs> {
    return [];
  }
}
