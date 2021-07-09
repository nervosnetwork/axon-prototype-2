import { WitnessInputType } from "./interfaces/witness_input_type";
import { remove0xPrefix, Uint8BigIntToLeHex } from "../../../utils/tools";

/*
    pub pattern:  u8,
    pub chain_id: u8,
 */
export class AdminCreateSidechainWitness implements WitnessInputType {
  static ADMIN_CREATE_SIDECHAIN_WITNESS = 2n;

  pattern: bigint;
  chainId: bigint;

  constructor(chainId: bigint) {
    this.pattern = AdminCreateSidechainWitness.ADMIN_CREATE_SIDECHAIN_WITNESS;
    this.chainId = chainId;
  }

  static default(): AdminCreateSidechainWitness {
    return new AdminCreateSidechainWitness(0n);
  }

  toWitness(): CKBComponents.WitnessArgs {
    const data = `0x${remove0xPrefix(Uint8BigIntToLeHex(this.pattern))}${remove0xPrefix(
      Uint8BigIntToLeHex(this.chainId),
    )}`;

    return { lock: "", inputType: data, outputType: "" };
  }
}
