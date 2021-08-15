import { Cell, OutPoint } from "@ckb-lumos/base";
import {
  defaultOutPoint,
  remove0xPrefix,
  Uint64BigIntToLeHex,
  scriptArgToArrayBuff,
  arrayBufferToHex,
} from "../../../utils/tools";
import { CellOutputType } from "./interfaces/cell_output_type";
import { CellInputType } from "./interfaces/cell_input_type";
import { SIDECHAIN_CONFIG_LOCK_SCRIPT, SIDECHAIN_CONFIG_TYPE_SCRIPT } from "../../../utils/environment";
import { CellDepType } from "./interfaces/cell_dep_type";
import {
  arrayBufferToBlockHeight,
  arrayBufferToBytes1,
  arrayBufferToChainId,
  arrayBufferToCodeHash,
  arrayBufferToHashType,
  arrayBufferToPublicKeyHash,
  arrayBufferToUint128,
  arrayBufferToUint32,
  arrayBufferToUint8,
  blockHeightToArrayBuffer,
  bytes1ToArrayBuffer,
  codeHashToArrayBuffer,
  hashTypeToArrayBuffer,
  publicKeyHashToArrayBuffer,
  uint128ToArrayBuffer,
  uint32ToArrayBuffer,
  uint8ToArrayBuffer,
} from "../../../utils/mol";
import {
  SerializeSidechainConfigCell,
  SidechainConfigCell,
  SidechainConfigCellTypeArgs,
} from "../mol/sidechain_config";

/*
sidechain config

capacity: - 8 bytes
data: -
    pub chain_id:                u8,
    pub checkerTotalCount:     u8,
    // 2**8 = 256
    pub checkerBitmap:          [u8; 32],
    // 256
    pub checkerThreshold:       u8,
    pub updateInterval:         u16,
    pub minimalBond:            u128,
    pub checkerDataSizeLimit: u128,
    pub checkerPrice:           u128,
    pub refreshInterval:        u16,
    pub commitThreshold:        u8,
    pub challengeThreshold:     u8,
    pub adminPublicKey:        [u8; 32],
    pub collatorPublicKey:     [u8; 32],
    pub bondSudtTypeHash:     [u8; 32],
type: - 65 bytes
    codehash: type id
    hashtype: type
    args: chain id
lock: - A.S. 33 bytes

 */
export class SidechainConfig implements CellInputType, CellOutputType, CellDepType {
  capacity: bigint;

  sidechainStatus: string;

  commitThreshold: bigint;
  challengeThreshold: bigint;

  checkerNormalCount: bigint;
  checkerThreshold: bigint;
  checkerTotalCount: bigint;
  checkers: Array<{ lock_arg: string; status: string }>;

  refreshPunishPoints: bigint;
  refreshPunishReleasePoints: bigint;
  refreshPunishThreshold: bigint;
  refreshSidechainHeightInterval: string;

  checkDataSizeLimit: bigint;
  checkFeeRate: bigint;
  minimalBond: bigint;
  parallelJobUpperBond: bigint;
  parallelJobMaximalHeightRange: string;

  adminLockArg: string;
  collatorLockArg: string;

  bondSudtTypescriptCodehash: string;
  bondSudtTypescriptHashtype: string;

  //type args

  chainId: string;

  outPoint: OutPoint;

  constructor(
    capacity: bigint,
    sidechainStatus: string,
    commitThreshold: bigint,
    challengeThreshold: bigint,
    checkerNormalCount: bigint,
    checkerThreshold: bigint,
    checkerTotalCount: bigint,
    checkers: Array<{ lock_arg: string; status: string }>,
    refreshPunishPoints: bigint,
    refreshPunishReleasePoints: bigint,
    refreshPunishThreshold: bigint,
    refreshSidechainHeightInterval: string,
    checkDataSizeLimit: bigint,
    checkFeeRate: bigint,
    minimalBond: bigint,
    parallelJobUpperBond: bigint,
    parallelJobMaximalHeightRange: string,
    adminLockArg: string,
    collatorLockArg: string,
    bondSudtTypescriptCodehash: string,
    bondSudtTypescriptHashtype: string,
    chainId: string,
    outPoint: OutPoint,
  ) {
    this.capacity = capacity;
    this.sidechainStatus = sidechainStatus;
    this.commitThreshold = commitThreshold;
    this.challengeThreshold = challengeThreshold;
    this.checkerNormalCount = checkerNormalCount;
    this.checkerThreshold = checkerThreshold;
    this.checkerTotalCount = checkerTotalCount;
    this.checkers = checkers;
    this.refreshPunishPoints = refreshPunishPoints;
    this.refreshPunishReleasePoints = refreshPunishReleasePoints;
    this.refreshPunishThreshold = refreshPunishThreshold;
    this.refreshSidechainHeightInterval = refreshSidechainHeightInterval;
    this.checkDataSizeLimit = checkDataSizeLimit;
    this.checkFeeRate = checkFeeRate;
    this.minimalBond = minimalBond;
    this.parallelJobUpperBond = parallelJobUpperBond;
    this.parallelJobMaximalHeightRange = parallelJobMaximalHeightRange;
    this.adminLockArg = adminLockArg;
    this.collatorLockArg = collatorLockArg;
    this.bondSudtTypescriptCodehash = bondSudtTypescriptCodehash;
    this.bondSudtTypescriptHashtype = bondSudtTypescriptHashtype;
    this.chainId = chainId;
    this.outPoint = outPoint;
  }

  static validate(cell: Cell): boolean {
    if (!cell.out_point) {
      return false;
    }

    return true;
  }

  static fromCell(cell: Cell): SidechainConfig | null {
    if (!SidechainConfig.validate(cell)) {
      return null;
    }
    const capacity = BigInt(cell.cell_output.capacity);

    const cellData = new SidechainConfigCell(Buffer.from(remove0xPrefix(cell.data), "hex").buffer, { validate: true });

    const sidechainStatus = arrayBufferToBytes1(cellData.getSidechainStatus().raw());

    const commitThreshold = arrayBufferToUint32(cellData.getCommitThreshold().raw());
    const challengeThreshold = arrayBufferToUint32(cellData.getCommitThreshold().raw());

    const checkerNormalCount = arrayBufferToUint32(cellData.getCheckerNormalCount().raw());
    const checkerThreshold = arrayBufferToUint32(cellData.getCheckerThreshold().raw());
    const checkerTotalCount = arrayBufferToUint32(cellData.getCheckerTotalCount().raw());
    const checkers: Array<{ lock_arg: string; status: string }> = [];

    for (let i = 0; i < cellData.getCheckers().length(); i++) {
      const item = cellData.getCheckers().indexAt(i);

      checkers.push({
        lock_arg: arrayBufferToPublicKeyHash(item.getLockArg().raw()),
        status: arrayBufferToBytes1(item.getStatus().raw()),
      });
    }

    const refreshPunishPoints = arrayBufferToUint32(cellData.getRefreshPunishPoints().raw());
    const refreshPunishReleasePoints = arrayBufferToUint32(cellData.getRefreshPunishReleasePoints().raw());
    const refreshPunishThreshold = arrayBufferToUint32(cellData.getRefreshPunishThreshold().raw());
    const refreshSidechainHeightInterval = arrayBufferToBlockHeight(cellData.getRefreshSidechainHeightInterval().raw());

    const checkDataSizeLimit = arrayBufferToUint128(cellData.getCheckDataSizeLimit().raw());
    const checkFeeRate = arrayBufferToUint32(cellData.getCheckFeeRate().raw());
    const minimalBond = arrayBufferToUint128(cellData.getMinimalBond().raw());
    const parallelJobUpperBond = arrayBufferToUint8(cellData.getParallelJobUpperBond().raw());
    const parallelJobMaximalHeightRange = arrayBufferToBlockHeight(cellData.getParallelJobMaximalHeightRange().raw());

    const adminLockArg = arrayBufferToPublicKeyHash(cellData.getAdminLockArg().raw());
    const collatorLockArg = arrayBufferToPublicKeyHash(cellData.getCollatorLockArg().raw());

    const bondSudtTypescriptCodehash = arrayBufferToCodeHash(cellData.getBondSudtTypescriptCodehash().raw());
    const bondSudtTypescriptHashtype = arrayBufferToHashType(cellData.getBondSudtTypescriptHashtype().raw());

    //==============================================================================

    const typeArgs = new SidechainConfigCellTypeArgs(scriptArgToArrayBuff(cell.cell_output.type!), { validate: true });

    const chainId = arrayBufferToChainId(typeArgs.getChainId().raw());

    const outPoint = cell.out_point!;

    return new SidechainConfig(
      capacity,
      sidechainStatus,
      commitThreshold,
      challengeThreshold,
      checkerNormalCount,
      checkerThreshold,
      checkerTotalCount,
      checkers,
      refreshPunishPoints,
      refreshPunishReleasePoints,
      refreshPunishThreshold,
      refreshSidechainHeightInterval,
      checkDataSizeLimit,
      checkFeeRate,
      minimalBond,
      parallelJobUpperBond,
      parallelJobMaximalHeightRange,
      adminLockArg,
      collatorLockArg,
      bondSudtTypescriptCodehash,
      bondSudtTypescriptHashtype,
      chainId,
      outPoint,
    );
  }

  static default(): SidechainConfig {
    return new SidechainConfig(
      0n,
      "",
      0n,
      0n,
      0n,
      0n,
      0n,
      [],
      0n,
      0n,
      0n,
      ``,
      0n,
      0n,
      0n,
      0n,
      ``,
      ``,
      ``,
      ``,
      ``,
      ``,
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
      type: SIDECHAIN_CONFIG_TYPE_SCRIPT,
      lock: SIDECHAIN_CONFIG_LOCK_SCRIPT,
    };
  }

  toCellOutputData(): string {
    const checkers = this.checkers.map((checker) => {
      publicKeyHashToArrayBuffer(checker.lock_arg);
      bytes1ToArrayBuffer(checker.status);
    });

    const sidechainConfigCell = {
      sidechainStatus: bytes1ToArrayBuffer(this.sidechainStatus),

      commitThreshold: uint32ToArrayBuffer(this.commitThreshold),
      challengeThreshold: uint32ToArrayBuffer(this.challengeThreshold),

      checkerNormalCount: uint32ToArrayBuffer(this.checkerNormalCount),
      checkerThreshold: uint32ToArrayBuffer(this.checkerThreshold),
      checkerTotalCount: uint32ToArrayBuffer(this.checkerTotalCount),
      checkers: checkers,

      refreshPunishPoints: uint32ToArrayBuffer(this.refreshPunishPoints),
      refreshPunishReleasePoints: uint32ToArrayBuffer(this.refreshPunishReleasePoints),
      refreshPunishThreshold: uint32ToArrayBuffer(this.refreshPunishThreshold),
      refreshSidechainHeightInterval: blockHeightToArrayBuffer(this.refreshSidechainHeightInterval),

      checkDataSizeLimit: uint128ToArrayBuffer(this.checkDataSizeLimit),
      checkFeeRate: uint32ToArrayBuffer(this.checkFeeRate),
      minimalBond: uint128ToArrayBuffer(this.minimalBond),
      parallelJobUpperBond: uint8ToArrayBuffer(this.parallelJobUpperBond),
      parallelJobMaximalHeightRange: blockHeightToArrayBuffer(this.parallelJobMaximalHeightRange),

      adminLockArg: publicKeyHashToArrayBuffer(this.adminLockArg),
      collatorLockArg: publicKeyHashToArrayBuffer(this.collatorLockArg),

      bondSudtTypescriptCodehash: codeHashToArrayBuffer(this.bondSudtTypescriptCodehash),
      bondSudtTypescriptHashtype: hashTypeToArrayBuffer(this.bondSudtTypescriptHashtype),
    };
    return arrayBufferToHex(SerializeSidechainConfigCell(sidechainConfigCell));
  }

  getOutPoint(): string {
    return `${this.outPoint.tx_hash}-${this.outPoint.index}`;
  }

  static fromJSON(source: unknown): SidechainConfig {
    return Object.assign(SidechainConfig.default(), source);
  }
}
