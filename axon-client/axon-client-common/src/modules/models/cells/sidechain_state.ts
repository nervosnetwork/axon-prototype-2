import { Cell, OutPoint } from "@ckb-lumos/base";
import {
  arrayBufferToHex,
  defaultOutPoint,
  remove0xPrefix,
  scriptArgToArrayBuff,
  Uint64BigIntToLeHex,
} from "../../../utils/tools";
import { CellOutputType } from "./interfaces/cell_output_type";
import { CellInputType } from "./interfaces/cell_input_type";
import { SIDECHAIN_STATE_LOCK_SCRIPT, SIDECHAIN_STATE_TYPE_SCRIPT } from "../../../utils/environment";
import { CellDepType } from "./interfaces/cell_dep_type";
import {
  arrayBufferToBlockHeader,
  arrayBufferToBlockHeight,
  arrayBufferToChainId,
  arrayBufferToCommittedHash,
  arrayBufferToMerkleHash,
  arrayBufferToPublicKeyHash,
  arrayBufferToRandomSeed,
  arrayBufferToUint32,
  arrayBufferToUint8,
  blockHeaderToArrayBuffer,
  blockHeightToArrayBuffer,
  committedHashToArrayBuffer,
  merkleHashToArrayBuffer,
  publicKeyHashToArrayBuffer,
  randomSeedToArrayBuffer,
  uint32ToArrayBuffer,
  uint8ToArrayBuffer,
} from "../../../utils/mol";
import { SerializeSidechainStateCell, SidechainStateCell, SidechainStateCellLockArgs } from "../mol/sidechain_state";

/*
sidechain status

capacity: - 8 bytes
data: -
    pub chain_id:               u8,
    pub version:                u8,
    pub latest_block_height:    u128,
    pub latest_block_hash:      [u8; 32],
    pub committed_block_height: u128,
    pub committed_block_hash:   [u8; 32],
    pub status:                 u8,
type: - 65 bytes
    codehash: type id
    hashtype: type
    args: chain id
lock: - A.S. 33 bytes

 */
export class SidechainState implements CellInputType, CellOutputType, CellDepType {
  // 0 for waiting publish task
  // 1 for waiting submit task/challenge, the task is published
  static STATUS_WAITING_FOR_PUBLISH = BigInt(0);
  static STATUS_WAITING_FOR_SUBMIT = BigInt(1);

  capacity: bigint;

  version: bigint;
  submitSidechainBlockHeight: bigint;
  waitingJobs: Array<{ from: bigint; to: bigint }>;
  confirmedJobs: Array<{ from: bigint; to: bigint }>;
  randomSeed: string;
  randomOffset: bigint;
  randomCommit: { checker_lock_arg: string; committed_hash: string };
  punishCheckers: Array<{ checker_lock_arg: string; punish_points: bigint }>;
  recentBlockHeaders: Array<string>;
  ancientBlockHeardMerkleRoot: string;
  checkerLastTaskSidechainHeights: Array<{ checker_lock_arg: string; height: bigint }>;

  //args
  chainId: bigint;

  outPoint: OutPoint;

  constructor(
    capacity: bigint,
    version: bigint,
    submitSidechainBlockHeight: bigint,
    waitingJobs: Array<{ from: bigint; to: bigint }>,
    confirmedJobs: Array<{ from: bigint; to: bigint }>,
    randomSeed: string,
    randomOffset: bigint,
    randomCommit: { checker_lock_arg: string; committed_hash: string },
    punishCheckers: Array<{ checker_lock_arg: string; punish_points: bigint }>,
    recentBlockHeaders: Array<string>,
    ancientBlockHeardMerkleRoot: string,
    checkerLastTaskSidechainHeights: Array<{ checker_lock_arg: string; height: bigint }>,
    chainId: bigint,
    outPoint: OutPoint,
  ) {
    this.capacity = capacity;
    this.version = version;
    this.submitSidechainBlockHeight = submitSidechainBlockHeight;
    this.waitingJobs = waitingJobs;
    this.confirmedJobs = confirmedJobs;
    this.randomSeed = randomSeed;
    this.randomOffset = randomOffset;
    this.randomCommit = randomCommit;
    this.punishCheckers = punishCheckers;
    this.recentBlockHeaders = recentBlockHeaders;
    this.ancientBlockHeardMerkleRoot = ancientBlockHeardMerkleRoot;
    this.checkerLastTaskSidechainHeights = checkerLastTaskSidechainHeights;
    this.chainId = chainId;
    this.outPoint = outPoint;
  }

  static validate(cell: Cell): boolean {
    if (!cell.out_point) {
      return false;
    }

    return true;
  }

  static fromCell(cell: Cell): SidechainState | null {
    if (!SidechainState.validate(cell)) {
      return null;
    }
    const capacity = BigInt(cell.cell_output.capacity);

    const cellData = new SidechainStateCell(Buffer.from(remove0xPrefix(cell.data), "hex").buffer, { validate: true });

    const version = arrayBufferToUint8(cellData.getVersion().raw());
    const submitSidechainBlockHeight = arrayBufferToBlockHeight(cellData.getSubmitSidechainBlockHeight().raw());
    const waitingJobs: Array<{ from: bigint; to: bigint }> = [];

    for (let i = 0; i < cellData.getWaitingJobs().length(); i++) {
      const item = cellData.getWaitingJobs().indexAt(i);
      waitingJobs.push({
        from: arrayBufferToBlockHeight(item.getFrom().raw()),
        to: arrayBufferToBlockHeight(item.getTo().raw()),
      });
    }

    const confirmedJobs: Array<{ from: bigint; to: bigint }> = [];
    for (let i = 0; i < cellData.getConfirmedJobs().length(); i++) {
      const item = cellData.getConfirmedJobs().indexAt(i);
      confirmedJobs.push({
        from: arrayBufferToBlockHeight(item.getFrom().raw()),
        to: arrayBufferToBlockHeight(item.getTo().raw()),
      });
    }

    const randomSeed = arrayBufferToRandomSeed(cellData.getRandomSeed().raw());
    const randomOffset = arrayBufferToUint8(cellData.getRandomOffset().raw());
    const randomCommit = {
      checker_lock_arg: arrayBufferToPublicKeyHash(cellData.getRandomCommit().getCheckerLockArg().raw()),
      committed_hash: arrayBufferToCommittedHash(cellData.getRandomCommit().getCommittedHash().raw()),
    };

    const punishCheckers: Array<{ checker_lock_arg: string; punish_points: bigint }> = [];
    for (let i = 0; i < cellData.getPunishCheckers().length(); i++) {
      const item = cellData.getPunishCheckers().indexAt(i);
      punishCheckers.push({
        checker_lock_arg: arrayBufferToPublicKeyHash(item.getCheckerLockArg().raw()),
        punish_points: arrayBufferToUint32(item.getPunishPoints().raw()),
      });
    }

    const recentBlockHeaders: Array<string> = [];
    for (let i = 0; i < cellData.getRecentBlockHeaders().length(); i++) {
      const item = cellData.getRecentBlockHeaders().indexAt(i);
      recentBlockHeaders.push(arrayBufferToBlockHeader(item.raw()));
    }

    const ancientBlockHeardMerkleRoot: string = arrayBufferToMerkleHash(
      cellData.getAncientBlockHeardMerkleRoot().raw(),
    );
    const checkerLastTaskSidechainHeights: Array<{ checker_lock_arg: string; height: bigint }> = [];
    for (let i = 0; i < cellData.getCheckerLastTaskSidechainHeights().length(); i++) {
      const item = cellData.getCheckerLastTaskSidechainHeights().indexAt(i);
      checkerLastTaskSidechainHeights.push({
        checker_lock_arg: arrayBufferToPublicKeyHash(item.getCheckerLockArg().raw()),
        height: arrayBufferToBlockHeight(item.getHeight().raw()),
      });
    }

    //==============================

    if (!cell.cell_output.type) {
      return null;
    }

    const typeArgs = new SidechainStateCellLockArgs(scriptArgToArrayBuff(cell.cell_output.type), { validate: true });

    const chainId = arrayBufferToChainId(typeArgs.getChainId().raw());

    const outPoint = cell.out_point!;

    return new SidechainState(
      capacity,

      version,
      submitSidechainBlockHeight,
      waitingJobs,
      confirmedJobs,
      randomSeed,
      randomOffset,
      randomCommit,
      punishCheckers,
      recentBlockHeaders,
      ancientBlockHeardMerkleRoot,
      checkerLastTaskSidechainHeights,
      //args
      chainId,
      outPoint,
    );
  }

  static default(): SidechainState {
    return new SidechainState(
      0n,
      0n,
      0n,
      [],
      [],
      ``,
      0n,
      { checker_lock_arg: "", committed_hash: "" },
      [],
      [],
      "",
      [],
      0n,
      defaultOutPoint(),
    );
  }

  toCellDep(): CKBComponents.CellDep {
    return {
      outPoint: {
        txHash: this.outPoint.tx_hash,
        index: this.outPoint.index,
      },
      depType: "code",
    };
  }

  toCellInput(): CKBComponents.CellInput {
    return {
      previousOutput: {
        txHash: this.outPoint.tx_hash,
        index: this.outPoint.index,
      },
      since: "0x0",
    };
  }

  toCellOutput(): CKBComponents.CellOutput {
    //skip change chainId
    return {
      capacity: Uint64BigIntToLeHex(this.capacity),
      type: SIDECHAIN_STATE_TYPE_SCRIPT,
      lock: SIDECHAIN_STATE_LOCK_SCRIPT,
    };
  }

  toCellOutputData(): string {
    const sidechainStateCell = {
      version: uint8ToArrayBuffer(this.version),
      submitSidechainBlockHeight: blockHeightToArrayBuffer(this.submitSidechainBlockHeight),
      waitingJobs: this.waitingJobs.map((job) => {
        return {
          from: blockHeightToArrayBuffer(job.from),
          to: blockHeightToArrayBuffer(job.to),
        };
      }),
      confirmedJobs: this.confirmedJobs.map((job) => {
        return {
          from: blockHeightToArrayBuffer(job.from),
          to: blockHeightToArrayBuffer(job.to),
        };
      }),
      randomSeed: randomSeedToArrayBuffer(this.randomSeed),
      randomOffset: uint8ToArrayBuffer(this.randomOffset),
      randomCommit: {
        checker_lock_arg: publicKeyHashToArrayBuffer(this.randomCommit.checker_lock_arg),
        committed_hash: committedHashToArrayBuffer(this.randomCommit.committed_hash),
      },
      punishCheckers: this.punishCheckers.map((checker) => {
        return {
          checker_lock_arg: publicKeyHashToArrayBuffer(checker.checker_lock_arg),
          punish_points: uint32ToArrayBuffer(checker.punish_points),
        };
      }),
      recentBlockHeaders: this.recentBlockHeaders.map((header) => blockHeaderToArrayBuffer(header)),
      ancientBlockHeardMerkleRoot: merkleHashToArrayBuffer(this.ancientBlockHeardMerkleRoot),
      checkerLastTaskSidechainHeights: this.checkerLastTaskSidechainHeights.map((height) => {
        return {
          checker_lock_arg: publicKeyHashToArrayBuffer(height.checker_lock_arg),
          height: blockHeightToArrayBuffer(height.height),
        };
      }),
    };

    return arrayBufferToHex(SerializeSidechainStateCell(sidechainStateCell));
  }

  getOutPoint(): string {
    return `${this.outPoint.tx_hash}-${this.outPoint.index}`;
  }

  static fromJSON(source: unknown): SidechainState {
    return Object.assign(SidechainState.default(), source);
  }
}
