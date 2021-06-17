import {Transformation} from './interfaces/transformation'
import {GlobalConfig} from "../cells/global_config";
import {Code} from "../cells/code";
import {CheckerInfo} from "../cells/checker_info";
import {SidechainFee} from "../cells/sidechain_fee";
import {Muse} from "../cells/muse";
import {CheckerTakeBeneficiaryWitness} from "../witnesses/checker_take_beneficiary_witness";

/*
CheckerTakeBeneficiary,

Dep:    0 Global Config Cell

Code Cell                   ->         Code Cell
Checker Info Cell           ->          Checker Info Cell
Sidechain Fee Cell          ->          Sidechain Fee Cell
Muse Token Cell             ->          Muse Token Cell

*/
export class CheckTakeBeneficiaryTransformation implements Transformation {

    depGlobalConfig: GlobalConfig

    //use outpoint to refer as input
    //update cell and use it as output
    inputCode: Code
    inputCheckerInfo: CheckerInfo
    inputSidechainFee: SidechainFee
    inputMuse: Muse

    patternTypeWitness: CheckerTakeBeneficiaryWitness | null = null

    processed: boolean = false;
    skip: boolean = false;
    composedTx?: CKBComponents.RawTransaction = undefined
    composedTxHash?: string = undefined

    constructor(depGlobalConfig: GlobalConfig,
                inputCode: Code,
                inputCheckerInfo: CheckerInfo,
                inputSidechainFee: SidechainFee,
                inputMuse: Muse
    ) {
        this.depGlobalConfig = depGlobalConfig;
        this.inputCode = inputCode;
        this.inputCheckerInfo = inputCheckerInfo;
        this.inputSidechainFee = inputSidechainFee;
        this.inputMuse = inputMuse;
    }

    toCellDeps(): Array<CKBComponents.CellDep> {
        return [
            this.depGlobalConfig.toCellDep(),
        ];
    }

    toCellInput(): Array<CKBComponents.CellInput> {
        return [
            this.inputCode.toCellInput(),
            this.inputCheckerInfo.toCellInput(),
            this.inputSidechainFee.toCellInput(),
            this.inputMuse.toCellInput(),
        ]
    }

    toCellOutput(): Array<CKBComponents.CellOutput> {
        return [
            this.inputCode.toCellOutput(),
            this.inputCheckerInfo.toCellOutput(),
            this.inputSidechainFee.toCellOutput(),
            this.inputMuse.toCellOutput(),

        ]
    }

    toCellOutputData(): Array<string> {

        return [
            this.inputCode.toCellOutputData(),
            this.inputCheckerInfo.toCellOutputData(),
            this.inputSidechainFee.toCellOutputData(),
            this.inputMuse.toCellOutputData(),
        ]
    }

    toWitness(): Array<CKBComponents.WitnessArgs> {
        return [
            this.patternTypeWitness!.toWitness()
        ];
    }


}
