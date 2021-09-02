// mark a cell could be a CellOutput while Transformation
export interface CellOutputType {
  capacity: bigint;

  toCellOutput(): CKBComponents.CellOutput;

  toCellOutputData(): string;
}
