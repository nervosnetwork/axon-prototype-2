// mark a cell could be a CellOutput while Transformation
export interface CellOutputType {
    toCellOutput(): CKBComponents.CellOutput

    toCellOutputData(): string
}
