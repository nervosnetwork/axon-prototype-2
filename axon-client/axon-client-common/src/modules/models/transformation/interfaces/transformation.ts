// mark a transformation

export interface Transformation {
    // this Transformation is fully processed and all fields are up-to-date, and Output cells are generated
    processed: boolean
    // skip this Transformation since the Transformation fails according to business logic
    skip: boolean

    composedTx?: CKBComponents.RawTransaction

    composedTxHash?: string

    toCellDeps(): Array<CKBComponents.CellDep>

    toCellInput(): Array<CKBComponents.CellInput>

    toCellOutput(): Array<CKBComponents.CellOutput>

    toCellOutputData(): Array<string>

    toWitness(): Array<CKBComponents.WitnessArgs>

}
