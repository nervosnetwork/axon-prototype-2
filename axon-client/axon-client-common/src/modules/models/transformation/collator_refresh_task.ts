import {Transformation} from './interfaces/transformation'
import {GlobalConfig} from "../cells/global_config";
import {SidechainConfig} from "../cells/sidechain_config";
import {Code} from "../cells/code";
import {Task} from "../cells/task";
import {CollatorRefreshTaskWitness} from "../witnesses/collator_refresh_task_witness";


/*
CollatorRefreshTask,

Dep:    0 Global Config Cell
Dep:    1 Sidechain Config Cell

Code Cell                   ->          Code Cell
[Task Cell]                 ->          [Task Cell]

*/

export class CollatorRefreshTaskTransformation implements Transformation {

    depGlobalConfig: GlobalConfig
    depConfig: SidechainConfig

    //use outpoint to refer as input
    //update cell and use it as output
    inputCode: Code
    inputTasks: Array<Task>

    patternTypeWitness: CollatorRefreshTaskWitness | null


    processed: boolean = false;
    skip: boolean = false;
    composedTx?: CKBComponents.RawTransaction = undefined
    composedTxHash?: string = undefined

    constructor(depGlobalConfig: GlobalConfig,
                depConfig: SidechainConfig,
                inputCode: Code,
                inputTasks: Array<Task>
    ) {
        this.depGlobalConfig = depGlobalConfig;
        this.depConfig = depConfig;
        this.inputCode = inputCode;
        this.inputTasks = inputTasks;
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
            ...this.inputTasks.map(task => task.toCellInput())
        ]
    }

    toCellOutput(): Array<CKBComponents.CellOutput> {
        return [
            this.inputCode.toCellOutput(),
            ...this.inputTasks.map(task => task.toCellOutput())

        ]
    }

    toCellOutputData(): Array<string> {

        return [
            this.inputCode.toCellOutputData(),
            ...this.inputTasks.map(task => task.toCellOutputData())
        ]
    }

    toWitness(): Array<CKBComponents.WitnessArgs> {
        return [
            this.patternTypeWitness!.toWitness()
        ];
    }


}
