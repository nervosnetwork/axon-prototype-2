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
import { TASK_STATE_LOCK_SCRIPT, TASK_TYPE_SCRIPT } from "../../../utils/environment";
import { SerializeTaskCell, TaskCell, TaskCellTypeArgs } from "../mol/cellData/task";
import {
  arrayBufferToBlockHeader,
  arrayBufferToBlockHeight,
  arrayBufferToChainId,
  arrayBufferToCommittedHash,
  arrayBufferToPublicKeyHash,
  arrayBufferToRandomSeed,
  arrayBufferToUint128,
  arrayBufferToUint8,
  blockHeaderToArrayBuffer,
  blockHeightToArrayBuffer,
  committedHashToArrayBuffer,
  randomSeedToArrayBuffer,
  uint128ToArrayBuffer,
  uint8ToArrayBuffer,
} from "../../../utils/mol";

/*
task

capacity: - 8 bytes
data: -
    pub chain_id:                u8,
    pub version:                 u8,
    pub check_block_height_from: u128,
    pub check_block_height_to:   u128,
    pub check_block_hash_to:     [u8;32],
    pub check_data_size:         u128,
    pub refresh_interval:        u16,
    pub mode:                    TaskCellMode,
type: - 65 bytes
    codehash: typeid
    hashtype: type
    args: chain id | public key hash - 21 bytes
lock: - A.S. 33 bytes

 */
export class Task implements CellInputType, CellOutputType {
  static TASK = 0n;
  static CHALLENGE = 1n;

  capacity: bigint;

  version: bigint;
  sidechainBlockHeightFrom: bigint;
  sidechainBlockHeightTo: bigint;
  checkDataSize: bigint;
  mode: bigint;
  status: bigint;
  reveal: string;
  commit: string;
  sidechainBlockHeader: Array<string>;

  chainId: bigint;
  checkerLockArg: string;

  outPoint: OutPoint;

  constructor(
    capacity: bigint,
    version: bigint,
    sidechainBlockHeightFrom: bigint,
    sidechainBlockHeightTo: bigint,
    checkDataSize: bigint,
    mode: bigint,
    status: bigint,
    reveal: string,
    commit: string,
    sidechainBlockHeader: Array<string>,
    chainId: bigint,
    checkerLockArg: string,
    outPoint: OutPoint,
  ) {
    this.capacity = capacity;
    this.version = version;
    this.sidechainBlockHeightFrom = sidechainBlockHeightFrom;
    this.sidechainBlockHeightTo = sidechainBlockHeightTo;
    this.checkDataSize = checkDataSize;
    this.mode = mode;
    this.status = status;
    this.reveal = reveal;
    this.commit = commit;
    this.sidechainBlockHeader = sidechainBlockHeader;
    this.chainId = chainId;
    this.checkerLockArg = checkerLockArg;
    this.outPoint = outPoint;
  }

  static validate(cell: Cell): boolean {
    if (!cell.out_point) {
      return false;
    }

    return true;
  }

  static fromCell(cell: Cell): Task | null {
    if (!Task.validate(cell)) {
      return null;
    }
    const capacity = BigInt(cell.cell_output.capacity);

    const cellData = new TaskCell(Buffer.from(remove0xPrefix(cell.data), "hex").buffer, { validate: true });

    const version = arrayBufferToUint8(cellData.getVersion().raw());
    const sidechainBlockHeightFrom = arrayBufferToBlockHeight(cellData.getSidechainBlockHeightFrom().raw());
    const sidechainBlockHeightTo = arrayBufferToBlockHeight(cellData.getSidechainBlockHeightTo().raw());
    const checkDataSize = arrayBufferToUint128(cellData.getCheckDataSize().raw());
    const mode = arrayBufferToUint8(cellData.getMode().raw());
    const status = arrayBufferToUint8(cellData.getStatus().raw());
    const reveal = arrayBufferToRandomSeed(cellData.getReveal().raw());
    const commit = arrayBufferToCommittedHash(cellData.getCommit().raw());
    const sidechainBlockHeader: Array<string> = [];
    for (let i = 0; i < cellData.getSidechainBlockHeader().length(); i++) {
      const item = cellData.getSidechainBlockHeader().indexAt(i);
      sidechainBlockHeader.push(arrayBufferToBlockHeader(item.raw()));
    }

    //==========================
    if (!cell.cell_output.type) {
      return null;
    }

    const typeArgs = new TaskCellTypeArgs(scriptArgToArrayBuff(cell.cell_output.type), { validate: true });

    const chainId = arrayBufferToChainId(typeArgs.getChainId().raw());
    const checkerLockArg = arrayBufferToPublicKeyHash(typeArgs.getCheckerLockArg().raw());

    const outPoint = cell.out_point!;

    return new Task(
      capacity,
      version,
      sidechainBlockHeightFrom,
      sidechainBlockHeightTo,
      checkDataSize,
      mode,
      status,
      reveal,
      commit,
      sidechainBlockHeader,

      chainId,
      checkerLockArg,
      outPoint,
    );
  }

  static default(): Task {
    return new Task(0n, 0n, 0n, 0n, 0n, 0n, 0n, ``, ``, [], 0n, ``, defaultOutPoint());
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
    //skip change chainId and checkerLockArg
    return {
      capacity: Uint64BigIntToLeHex(this.capacity),
      type: TASK_TYPE_SCRIPT,
      lock: TASK_STATE_LOCK_SCRIPT,
    };
  }

  toCellOutputData(): string {
    const taskCell = {
      version: uint8ToArrayBuffer(this.version),
      sidechain_block_height_from: blockHeightToArrayBuffer(this.sidechainBlockHeightFrom),
      sidechain_block_height_to: blockHeightToArrayBuffer(this.sidechainBlockHeightTo),
      check_data_size: uint128ToArrayBuffer(this.checkDataSize),
      mode: uint8ToArrayBuffer(this.mode),
      status: uint8ToArrayBuffer(this.status),
      reveal: randomSeedToArrayBuffer(this.reveal),
      commit: committedHashToArrayBuffer(this.commit),
      sidechain_block_header: this.sidechainBlockHeader.map((header) => blockHeaderToArrayBuffer(header)),
    };

    return arrayBufferToHex(SerializeTaskCell(taskCell));
  }

  getOutPoint(): string {
    return `${this.outPoint.tx_hash}-${this.outPoint.index}`;
  }

  static fromJSON(source: unknown): Task {
    return Object.assign(Task.default(), source);
  }
}
