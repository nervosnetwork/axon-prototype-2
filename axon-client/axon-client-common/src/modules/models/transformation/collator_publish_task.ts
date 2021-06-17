import {Transformation} from './interfaces/transformation'
import {GlobalConfig} from "../cells/global_config";
import {SidechainConfig} from "../cells/sidechain_config";
import {Code} from "../cells/code";
import {SidechainState} from "../cells/sidechain_state";
import {SidechainBond} from "../cells/sidechain_bond";
import {Task} from "../cells/task";
import {CollatorPublishTaskWitness} from "../witnesses/collator_publish_task_witness";

/*
CollatorPublishTask,

Dep:    0 Global Config Cell
Dep:    1 Sidechain Config Cell

Code Cell                   ->          Code Cell
Sidechain State Cell        ->          Sidechain State Cell
Sidechain Bond Cell/Sudt    ->          Sidechain Bond Cell
Null                        ->          [Task Cell]

*/

export class CollatorPublishTaskTransformation implements Transformation {

    depGlobalConfig: GlobalConfig
    depConfig: SidechainConfig

    //use outpoint to refer as input
    //update cell and use it as output
    inputCode: Code
    inputState: SidechainState
    inputBond: SidechainBond

    outputTask: Array<Task>

    patternTypeWitness: CollatorPublishTaskWitness | null


    processed: boolean = false;
    skip: boolean = false;
    composedTx?: CKBComponents.RawTransaction = undefined
    composedTxHash?: string = undefined

    constructor(depGlobalConfig: GlobalConfig,
                depSidechainConfig: SidechainConfig,
                inputCode: Code,
                inputState: SidechainState,
                inputBond: SidechainBond) {
        this.depGlobalConfig = depGlobalConfig;
        this.depConfig = depSidechainConfig;
        this.inputCode = inputCode;
        this.inputState = inputState;
        this.inputBond = inputBond;
        this.outputTask = [];
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
            this.inputBond.toCellInput(),
        ]
    }

    toCellOutput(): Array<CKBComponents.CellOutput> {
        return [
            this.inputCode.toCellOutput(),
            this.inputState.toCellOutput(),
            this.inputBond.toCellOutput(),
            ...this.outputTask.map(task => task.toCellOutput())
        ]
    }

    toCellOutputData(): Array<string> {

        return [
            this.inputCode.toCellOutputData(),
            this.inputState.toCellOutputData(),
            this.inputBond.toCellOutputData(),
            ...this.outputTask.map(task => task.toCellOutputData())
        ]
    }

    toWitness(): Array<CKBComponents.WitnessArgs> {
        return [
            this.patternTypeWitness!.toWitness()
        ];
    }


}
