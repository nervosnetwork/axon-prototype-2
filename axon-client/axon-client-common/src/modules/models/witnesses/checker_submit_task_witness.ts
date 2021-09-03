import { WitnessInputType } from "./interfaces/witness_input_type";
import { arrayBufferToHex, remove0xPrefix, Uint8BigIntToLeHex } from "../../../utils/tools";
import { publicKeyHashToArrayBuffer } from "../../../utils/mol";

/*
    pub pattern:    u8,
    pub chain_id:   u8,
    pub checker_lock_arg: string,
*/
export class CheckerVoteWitness implements WitnessInputType {
  static CHECKER_VOTE_WITNESS = 4n;

  pattern: bigint;
  chainId: bigint;
  checkerLockArg: string;

  constructor(chainId: bigint, checkerLockArg: string) {
    this.pattern = CheckerVoteWitness.CHECKER_VOTE_WITNESS;
    this.chainId = chainId;
    this.checkerLockArg = checkerLockArg;
  }

  static default(): CheckerVoteWitness {
    return new CheckerVoteWitness(0n, ``);
  }

  toWitness(): CKBComponents.WitnessArgs {
    const data = `0x${remove0xPrefix(Uint8BigIntToLeHex(this.pattern))}${remove0xPrefix(
      Uint8BigIntToLeHex(this.chainId),
    )}${remove0xPrefix(arrayBufferToHex(publicKeyHashToArrayBuffer(this.checkerLockArg)))}`;

    return { lock: "", inputType: data, outputType: "" };
  }
}
