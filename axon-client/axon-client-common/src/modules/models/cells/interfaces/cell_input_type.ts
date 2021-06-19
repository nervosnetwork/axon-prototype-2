// mark a cell could be a CellInput while Transformation
export interface CellInputType {
    toCellInput(): CKBComponents.CellInput

    // 0x???-0x?
    getOutPoint(): string
}
