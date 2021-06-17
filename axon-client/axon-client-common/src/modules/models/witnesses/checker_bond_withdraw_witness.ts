import {WitnessInputType} from "./interfaces/witness_input_type";
import {remove0xPrefix, Uint8BigIntToLeHex} from '../../../utils/tools'

/*
    pub pattern:  u8,
 */
export class CheckerBondWithdrawWitness implements WitnessInputType{

    static CHECKER_BOND_WITHDRAW_WITNESS = 4n

    pattern : bigint


    constructor() {
        this.pattern = CheckerBondWithdrawWitness.CHECKER_BOND_WITHDRAW_WITNESS;
    }

    static default(): CheckerBondWithdrawWitness {
        return new CheckerBondWithdrawWitness()
    }


    toWitness(): CKBComponents.WitnessArgs {
        let data = `0x${
            remove0xPrefix(Uint8BigIntToLeHex(this.pattern))}`

        return   {lock: '', inputType: data, outputType: ''}

    }

}
