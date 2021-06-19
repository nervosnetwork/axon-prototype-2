import {WitnessInputType} from "./interfaces/witness_input_type";
import {remove0xPrefix, Uint8BigIntToLeHex} from '../../../utils/tools'

/*
    pub pattern:               u8,
    pub chain_id:              u8,
*/
export class CollatorUnlockBondWitness implements WitnessInputType {

    static COLLATOR_UNLOCK_BOND_WITNESS = 15n

    pattern: bigint
    chainId: bigint

    constructor(chainId: bigint,) {
        this.pattern = CollatorUnlockBondWitness.COLLATOR_UNLOCK_BOND_WITNESS;
        this.chainId = chainId;
    }

    static default(): CollatorUnlockBondWitness {
        return new CollatorUnlockBondWitness(0n,)
    }


    toWitness(): CKBComponents.WitnessArgs {
        let data = `0x${
            remove0xPrefix(Uint8BigIntToLeHex(this.pattern))}${
            remove0xPrefix(Uint8BigIntToLeHex(this.chainId))}`

        return {lock: '', inputType: data, outputType: ''}

    }

}
