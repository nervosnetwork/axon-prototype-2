// mark a cell could be a CellInput while Transformation
export interface CellDepType {
  toCellDep(): CKBComponents.CellDep;

  // 0x???-0x?
  getOutPoint(): string;
}
