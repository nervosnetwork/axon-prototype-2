import { WitnessInputType } from "./interfaces/witness_input_type";
import { remove0xPrefix, Uint8BigIntToLeHex } from "../../../utils/tools";

/*
    pub pattern:    u8,
    pub chain_id:   u8,
    pub checker_id: u8,
*/
export class CheckerJoinSidechainWitness implements WitnessInputType {
  static CHECKER_JOIN_SIDECHAIN_WITNESS = 5n;

  pattern: bigint;
  chainId: bigint;
  checkerId: bigint;

  constructor(chainId: bigint, checkerId: bigint) {
    this.pattern = CheckerJoinSidechainWitness.CHECKER_JOIN_SIDECHAIN_WITNESS;
    this.chainId = chainId;
    this.checkerId = checkerId;
  }

  static default(): CheckerJoinSidechainWitness {
    return new CheckerJoinSidechainWitness(0n, 0n);
  }

  toWitness(): CKBComponents.WitnessArgs {
    const data = `0x${remove0xPrefix(Uint8BigIntToLeHex(this.pattern))}${remove0xPrefix(
      Uint8BigIntToLeHex(this.chainId),
    )}${remove0xPrefix(Uint8BigIntToLeHex(this.checkerId))}`;

    return { lock: "", inputType: data, outputType: "" };
  }
}
