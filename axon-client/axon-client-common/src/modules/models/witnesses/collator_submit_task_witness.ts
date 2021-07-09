import { WitnessInputType } from "./interfaces/witness_input_type";
import { remove0xPrefix, Uint128BigIntToLeHex, Uint8BigIntToLeHex } from "../../../utils/tools";

/*
    pub pattern:               u8,
    pub chain_id:              u8,
    pub fee:                   u128,
    pub fee_per_checker:       u128,
*/
export class CollatorSubmitTaskWitness implements WitnessInputType {
  static COLLATOR_SUBMIT_TASK_WITNESS = 12n;

  pattern: bigint;
  chainId: bigint;
  fee: bigint;
  feePerChecker: bigint;

  constructor(chainId: bigint, fee: bigint, feePerChecker: bigint) {
    this.pattern = CollatorSubmitTaskWitness.COLLATOR_SUBMIT_TASK_WITNESS;
    this.chainId = chainId;
    this.fee = fee;
    this.feePerChecker = feePerChecker;
  }

  static default(): CollatorSubmitTaskWitness {
    return new CollatorSubmitTaskWitness(0n, 0n, 0n);
  }

  toWitness(): CKBComponents.WitnessArgs {
    const data = `0x${remove0xPrefix(Uint8BigIntToLeHex(this.pattern))}${remove0xPrefix(
      Uint8BigIntToLeHex(this.chainId),
    )}${remove0xPrefix(Uint128BigIntToLeHex(this.fee))}${remove0xPrefix(Uint128BigIntToLeHex(this.feePerChecker))}`;

    return { lock: "", inputType: data, outputType: "" };
  }
}
