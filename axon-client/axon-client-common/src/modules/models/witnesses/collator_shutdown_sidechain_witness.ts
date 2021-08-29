import { uint8ToArrayBuffer } from "../../../utils/mol";
import { arrayBufferToHex } from "../../../utils/tools";
import { SerializeCollatorShutDownSidechainWitness } from "../mol/witness/collator_shutdown_sidechain";
import { WitnessInputType } from "./interfaces/witness_input_type";

export class CollatorShutDownSidechainWitness implements WitnessInputType {
  static COLLATOR_SHUTDOWN_SIDECHAIN_WITNESS = 12n;

  pattern: bigint;
  chainId: bigint;

  constructor(chainId: bigint) {
    this.pattern = CollatorShutDownSidechainWitness.COLLATOR_SHUTDOWN_SIDECHAIN_WITNESS;
    this.chainId = chainId;
  }

  static default() {
    return new CollatorShutDownSidechainWitness(0n);
  }

  toWitness(): CKBComponents.WitnessArgs {
    const witness = {
      pattern: uint8ToArrayBuffer(this.pattern),
      chain_id: uint8ToArrayBuffer(this.chainId),
    };
    const data = arrayBufferToHex(SerializeCollatorShutDownSidechainWitness(witness));

    return { lock: "", inputType: data, outputType: "" };
  }
}
