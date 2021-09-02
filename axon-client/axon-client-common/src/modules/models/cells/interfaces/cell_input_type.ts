// mark a cell could be a CellInput while Transformation
export interface CellInputType {
  capacity: bigint;

  toCellInput(): CKBComponents.CellInput;

  // 0x???-0x?
  getOutPoint(): string;
}
