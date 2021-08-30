import { publicKeyHashToArrayBuffer, uint128ToArrayBuffer, uint32ToArrayBuffer } from "../../../utils/mol";
import { Uint8BigIntToLeHex } from "../../../utils/tools";
import { SerializeAnyoneShutdownSidechainWitness } from "../mol/witness/anyone_shutdown_sidechain";
import { WitnessInputType } from "./interfaces/witness_input_type";

export class AnyoneShutDownSidechainWitness implements WitnessInputType {
  static ANYONE_SHUTDOWN_SIDECHAIN_WITNESS = 11n;

  pattern: bigint;
  challengeTimes: bigint;
  checkDataSize: bigint;
  jailedCheckers: Array<string>;

  constructor(challengeTimes: bigint, checkDataSize: bigint, jailedCheckers: Array<string>) {
    this.pattern = AnyoneShutDownSidechainWitness.ANYONE_SHUTDOWN_SIDECHAIN_WITNESS;
    this.challengeTimes = challengeTimes;
    this.checkDataSize = checkDataSize;
    this.jailedCheckers = jailedCheckers;
  }

  static default(): AnyoneShutDownSidechainWitness {
    return new AnyoneShutDownSidechainWitness(0n, 0n, []);
  }

  toWitness(): CKBComponents.WitnessArgs {
    const witness = {
      challenge_times: uint32ToArrayBuffer(this.challengeTimes),
      check_data_size: uint128ToArrayBuffer(this.checkDataSize),
      jailedCheckers: this.jailedCheckers.map((lock_arg) => publicKeyHashToArrayBuffer(lock_arg)),
    };
    const data = `${Uint8BigIntToLeHex(this.pattern)}${Buffer.from(
      SerializeAnyoneShutdownSidechainWitness(witness),
    ).toString("hex")}`;
    return { lock: "", inputType: data, outputType: "" };
  }
}
