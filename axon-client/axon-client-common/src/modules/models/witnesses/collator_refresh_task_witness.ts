import {WitnessInputType} from "./interfaces/witness_input_type";
import {remove0xPrefix, Uint8BigIntToLeHex} from '../../../utils/tools'

/*
    pub pattern:  u8,
    pub chain_id: u8,
*/
export class CollatorRefreshTaskWitness implements WitnessInputType{

    static COLLATOR_REFRESH_TASK_WITNESS = 14n

    pattern : bigint
    chainId : bigint


    constructor(chainId: bigint) {
        this.pattern = CollatorRefreshTaskWitness.COLLATOR_REFRESH_TASK_WITNESS;
        this.chainId = chainId;
    }

    static default(): CollatorRefreshTaskWitness {
        return new CollatorRefreshTaskWitness(0n)
    }


    toWitness(): CKBComponents.WitnessArgs {
        let data = `0x${
            remove0xPrefix(Uint8BigIntToLeHex(this.pattern))}${
            remove0xPrefix(Uint8BigIntToLeHex(this.chainId))}`

        return   {lock: '', inputType: data, outputType: ''}

    }

}
