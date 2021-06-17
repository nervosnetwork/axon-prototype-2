import {Transformation} from './interfaces/transformation'
import {GlobalConfig} from "../cells/global_config";
import {SidechainConfig} from "../cells/sidechain_config";
import {Code} from "../cells/code";
import {SidechainState} from "../cells/sidechain_state";
import {Ckb} from "../cells/ckb";
import {AdminCreateSidechainWitness} from "../witnesses/admin_create_sidechain_witness";

/*
AdminCreateSidechain,

Dep:    0 Global Config Cell

Code Cell                   ->          Code Cell
CKB Cell                    ->          Sidechain Config Cell
Null                        ->          Sidechain State Cell

*/

export class AdminCreateSidechainTransformation implements Transformation {

    depGlobalConfig: GlobalConfig

    //use outpoint to refer as input
    //update cell and use it as output
    inputCode: Code
    inputCkb: Ckb
    outputConfig: SidechainConfig | null
    outputState: SidechainState | null

    patternTypeWitness: AdminCreateSidechainWitness | null


    processed: boolean = false;
    skip: boolean = false;
    composedTx?: CKBComponents.RawTransaction = undefined
    composedTxHash?: string = undefined

    constructor(depGlobalConfig: GlobalConfig,
                inputCode: Code,
                inputCkb: Ckb,) {
        this.depGlobalConfig = depGlobalConfig;
        this.inputCode = inputCode;
        this.inputCkb = inputCkb;
        this.outputConfig = null;
        this.outputState = null;
        this.patternTypeWitness = null;
    }

    toCellDeps(): Array<CKBComponents.CellDep> {
        return [
            this.depGlobalConfig.toCellDep(),
        ];
    }

    toCellInput(): Array<CKBComponents.CellInput> {
        return [
            this.inputCode.toCellInput(),
            this.inputCkb.toCellInput(),
        ]
    }

    toCellOutput(): Array<CKBComponents.CellOutput> {
        return [
            this.inputCode.toCellOutput(),
            this.outputConfig!.toCellOutput(),
            this.outputState!.toCellOutput(),
        ]
    }

    toCellOutputData(): Array<string> {

        return [
            this.inputCode.toCellOutputData(),
            this.outputConfig!.toCellOutputData(),
            this.outputState!.toCellOutputData(),
        ]
    }

    toWitness(): Array<CKBComponents.WitnessArgs> {
        return [
            this.patternTypeWitness!.toWitness()
        ];
    }


}
