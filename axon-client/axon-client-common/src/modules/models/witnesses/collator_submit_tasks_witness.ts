import {
  committedHashOptToArrayBuffer,
  publicKeyHashToArrayBuffer,
  randomSeedToArrayBuffer,
  uint128ToArrayBuffer,
  uint32OptToArrayBuffer,
  uint32ToArrayBuffer,
} from "../../../utils/mol";
import { arrayBufferToHex, remove0xPrefix, Uint8BigIntToLeHex } from "../../../utils/tools";
import { SerializeCollatorSubmitTasksWitness } from "../mol/witness/collator_submit_tasks";
import { WitnessInputType } from "./interfaces/witness_input_type";

export class CollatorSubmitTasksWitness implements WitnessInputType {
  static COLLATOR_SUBMIT_TASKS_WITNESS = 8n;

  patetrn: bigint;
  challengeTimes: bigint;
  checkDataSize: bigint;
  commit: Array<{
    index: bigint | null;
    checkerLockArg: string;
    originCommitedHash: string | null;
    newCommittedHash: string | null;
  }>;
  originRandomSeed: string;
  newRandomSeed: string;

  constructor(
    challengTimes: bigint,
    checkDataSize: bigint,
    commit: Array<{
      index: bigint | null;
      checkerLockArg: string;
      originCommitedHash: string | null;
      newCommittedHash: string | null;
    }>,
    originRandomSeed: string,
    newRandomSeed: string,
  ) {
    this.patetrn = CollatorSubmitTasksWitness.COLLATOR_SUBMIT_TASKS_WITNESS;
    this.challengeTimes = challengTimes;
    this.checkDataSize = checkDataSize;
    this.commit = commit;
    this.originRandomSeed = originRandomSeed;
    this.newRandomSeed = newRandomSeed;
  }

  static default(): CollatorSubmitTasksWitness {
    return new CollatorSubmitTasksWitness(0n, 0n, [], ``, ``);
  }

  toWitness(): CKBComponents.WitnessArgs {
    const witness = {
      challenge_times: uint32ToArrayBuffer(this.challengeTimes),
      check_data_size: uint128ToArrayBuffer(this.checkDataSize),
      commit: this.commit.map((checkerInfo) => {
        return {
          index: uint32OptToArrayBuffer(checkerInfo.index),
          checker_lock_arg: publicKeyHashToArrayBuffer(checkerInfo.checkerLockArg),
          origin_committed_hash: committedHashOptToArrayBuffer(checkerInfo.originCommitedHash),
          new_committed_hash: committedHashOptToArrayBuffer(checkerInfo.newCommittedHash),
        };
      }),
      origin_random_seed: randomSeedToArrayBuffer(this.originRandomSeed),
      new_random_seed: randomSeedToArrayBuffer(this.newRandomSeed),
    };

    const data = `${Uint8BigIntToLeHex(this.patetrn)}${remove0xPrefix(
      arrayBufferToHex(SerializeCollatorSubmitTasksWitness(witness)),
    )}`;

    return { lock: "", inputType: data, outputType: "" };
  }
}
