import { WitnessInputType } from "./interfaces/witness_input_type";
import { remove0xPrefix, Uint128BigIntToLeHex, Uint8BigIntToLeHex } from "../../../utils/tools";

/*
    pub pattern:    u8,
    pub chain_id:   u8,
    pub checker_id: u8,
    pub fee:        u128,
*/
export class CheckerTakeBeneficiaryWitness implements WitnessInputType {
  static CHECKER_TAKE_BENEFICIARY_WITNESS = 10n;

  pattern: bigint;
  chainId: bigint;
  checkerId: bigint;
  fee: bigint;

  constructor(chainId: bigint, checkerId: bigint, fee: bigint) {
    this.pattern = CheckerTakeBeneficiaryWitness.CHECKER_TAKE_BENEFICIARY_WITNESS;
    this.chainId = chainId;
    this.checkerId = checkerId;
    this.fee = fee;
  }

  static default(): CheckerTakeBeneficiaryWitness {
    return new CheckerTakeBeneficiaryWitness(0n, 0n, 0n);
  }

  toWitness(): CKBComponents.WitnessArgs {
    const data = `0x${remove0xPrefix(Uint8BigIntToLeHex(this.pattern))}${remove0xPrefix(
      Uint8BigIntToLeHex(this.chainId),
    )}${remove0xPrefix(Uint8BigIntToLeHex(this.checkerId))}${remove0xPrefix(Uint128BigIntToLeHex(this.fee))}`;

    return { lock: "", inputType: data, outputType: "" };
  }
}
