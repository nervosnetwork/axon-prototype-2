import {Transformation} from './interfaces/transformation'
import {GlobalConfig} from "../cells/global_config";
import {SidechainConfig} from "../cells/sidechain_config";
import {Code} from "../cells/code";
import {CheckerInfo} from "../cells/checker_info";
import {CheckerBond} from "../cells/checker_bond";
import {CheckerJoinSidechainWitness} from "../witnesses/checker_join_sidechain_witness";

/*
CheckerJoinSidechain,

Dep:    0 Global Config Cell

Code Cell                   ->         Code Cell
Sidechain Config Cell       ->          Sidechain Config Cell
Checker Bond Cell           ->          Checker Bond Cell
Null                        ->          Checker Info Cell

*/

export class CheckJoinSidechainTransformation implements Transformation {

    depGlobalConfig: GlobalConfig

    //use outpoint to refer as input
    //update cell and use it as output
    inputCode: Code
    inputConfig: SidechainConfig
    inputCheckerBond: CheckerBond
    outputCheckerInfo: CheckerInfo | null

    patternTypeWitness: CheckerJoinSidechainWitness | null


    processed: boolean = false;
    skip: boolean = false;
    composedTx?: CKBComponents.RawTransaction = undefined
    composedTxHash?: string = undefined

    constructor(depGlobalConfig: GlobalConfig,
                inputCode: Code,
                inputConfig: SidechainConfig,
                inputCheckerBond: CheckerBond,
    ) {
        this.depGlobalConfig = depGlobalConfig;
        this.inputCode = inputCode;
        this.inputConfig = inputConfig;
        this.inputCheckerBond = inputCheckerBond;
        this.outputCheckerInfo = null;
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
            this.inputConfig.toCellInput(),
            this.inputCheckerBond.toCellInput(),
        ]
    }

    toCellOutput(): Array<CKBComponents.CellOutput> {
        return [
            this.inputCode.toCellOutput(),
            this.inputConfig.toCellOutput(),
            this.inputCheckerBond.toCellOutput(),
            this.outputCheckerInfo!.toCellOutput(),
        ]
    }

    toCellOutputData(): Array<string> {

        return [
            this.inputCode.toCellOutputData(),
            this.inputConfig!.toCellOutputData(),
            this.inputCheckerBond!.toCellOutputData(),
            this.outputCheckerInfo!.toCellOutputData(),

        ]
    }

    toWitness(): Array<CKBComponents.WitnessArgs> {
        return [
            this.patternTypeWitness!.toWitness()
        ];
    }


}
