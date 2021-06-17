import {Transformation} from './interfaces/transformation'
import {GlobalConfig} from "../cells/global_config";
import {SidechainConfig} from "../cells/sidechain_config";
import {Code} from "../cells/code";
import {SidechainState} from "../cells/sidechain_state";
import {SidechainFee} from "../cells/sidechain_fee";
import {CheckerInfo} from "../cells/checker_info";
import {CollatorSubmitTaskWitness} from "../witnesses/collator_submit_task_witness";

/*
CollatorSubmitTask,

Dep:    0 Global Config Cell
Dep:    1 Sidechain Config Cell

Code Cell                   ->          Code Cell
Sidechain State Cell        ->          Sidechain State Cell
Sidechain Fee Cell          ->          Sidechain Fee Cell
[Checker Info Cell]         ->          [Checker Info Cell]

*/

export class CollatorSubmitTaskTransformation implements Transformation {

    depGlobalConfig: GlobalConfig
    depConfig: SidechainConfig

    //use outpoint to refer as input
    //update cell and use it as output
    inputCode: Code
    inputState: SidechainState
    inputFee: SidechainFee
    inputCheckInfos: Array<CheckerInfo>

    patternTypeWitness: CollatorSubmitTaskWitness | null


    processed: boolean = false;
    skip: boolean = false;
    composedTx?: CKBComponents.RawTransaction = undefined
    composedTxHash?: string = undefined

    constructor(depGlobalConfig: GlobalConfig,
                depConfig: SidechainConfig,
                inputCode: Code,
                inputState: SidechainState,
                inputFee: SidechainFee,
                inputCheckInfos: Array<CheckerInfo>
    ) {
        this.depGlobalConfig = depGlobalConfig;
        this.depConfig = depConfig;
        this.inputCode = inputCode;
        this.inputState = inputState;
        this.inputFee = inputFee;
        this.inputCheckInfos = inputCheckInfos;
        this.patternTypeWitness = null;
    }

    toCellDeps(): Array<CKBComponents.CellDep> {
        return [
            this.depGlobalConfig.toCellDep(),
            this.depConfig.toCellDep(),
        ];
    }

    toCellInput(): Array<CKBComponents.CellInput> {
        return [
            this.inputCode.toCellInput(),
            this.inputState.toCellInput(),
            this.inputFee.toCellInput(),
            ...this.inputCheckInfos.map(ci => ci.toCellInput())
        ]
    }

    toCellOutput(): Array<CKBComponents.CellOutput> {
        return [
            this.inputCode.toCellOutput(),
            this.inputState.toCellOutput(),
            this.inputFee.toCellOutput(),
            ...this.inputCheckInfos.map(ci => ci.toCellOutput())
        ]
    }

    toCellOutputData(): Array<string> {

        return [
            this.inputCode.toCellOutputData(),
            this.inputState.toCellOutputData(),
            this.inputFee.toCellOutputData(),
            ...this.inputCheckInfos.map(ci => ci.toCellOutputData())
        ]
    }

    toWitness(): Array<CKBComponents.WitnessArgs> {
        return [
            this.patternTypeWitness!.toWitness()
        ];
    }


}
