import {WitnessInputType} from "./interfaces/witness_input_type";
import {remove0xPrefix, Uint128BigIntToLeHex, Uint8BigIntToLeHex} from '../../../utils/tools'

/*
    pub pattern:  u8,
    pub chain_id: u8,
    pub bond:     u128,
*/
export class CollatorPublishTaskWitness implements WitnessInputType{

    static COLLATOR_PUBLISH_TASK_WITNESS = 11n

    pattern : bigint
    chainId : bigint
    bond : bigint


    constructor(chainId: bigint, bond: bigint) {
        this.pattern = CollatorPublishTaskWitness.COLLATOR_PUBLISH_TASK_WITNESS;
        this.chainId = chainId;
        this.bond = bond;
    }

    static default(): CollatorPublishTaskWitness {
        return new CollatorPublishTaskWitness(0n,0n)
    }


    toWitness(): CKBComponents.WitnessArgs {
        let data = `0x${
            remove0xPrefix(Uint8BigIntToLeHex(this.pattern))}${
            remove0xPrefix(Uint8BigIntToLeHex(this.chainId))}${
            remove0xPrefix(Uint128BigIntToLeHex(this.bond))}`

        return   {lock: '', inputType: data, outputType: ''}

    }

}
