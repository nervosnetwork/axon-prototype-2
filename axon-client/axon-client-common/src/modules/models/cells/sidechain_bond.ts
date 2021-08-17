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
import { SIDECHAIN_BOND_LOCK_SCRIPT, SIDECHAIN_BOND_TYPE_SCRIPT } from "../../../utils/environment";
import { SerializeSudtTokenCell, SudtTokenCell } from "../mol/sudt_token";
import {
  arrayBufferToBlockHeight,
  arrayBufferToPublicKeyHash,
  arrayBufferToUint128,
  blockHeightToArrayBuffer,
  chainIdListToWrite,
  publicKeyHashToArrayBuffer,
  readerToChainIdList,
  uint128ToArrayBuffer,
} from "../../../utils/mol";
import { SerializeSidechainBondCellLockArgs, SidechainBondCellLockArgs } from "../mol/sidechain_bond";

/*
sidechain bond

capacity: - 8 bytes
data: - amount
type: - 65 bytes
    codehash: sudt type
    hashtype: type
    args: custom sudt admin
lock: - 65 bytes
    codehash: type id
    hashtype: type
    args: chain_id | collator_public_key_hash | unlock_sidechain_height - 37 bytes

 */
export class SidechainBond implements CellInputType, CellOutputType {
  capacity: bigint;
  sudtAmount: bigint;

  participatedChain: Array<bigint>;
  collatorLockArg: string;
  unlockSidechainHeight: bigint;

  outPoint: OutPoint;

  constructor(
    capacity: bigint,
    sudtAmount: bigint,
    participatedChain: Array<bigint>,
    collatorLockArg: string,
    unlockSidechainHeight: bigint,
    outPoint: OutPoint,
  ) {
    this.capacity = capacity;
    this.sudtAmount = sudtAmount;
    this.participatedChain = participatedChain;
    this.collatorLockArg = collatorLockArg;
    this.unlockSidechainHeight = unlockSidechainHeight;
    this.outPoint = outPoint;
  }

  static validate(cell: Cell): boolean {
    if (!cell.out_point) {
      return false;
    }

    return true;
  }

  static fromCell(cell: Cell): SidechainBond | null {
    if (!SidechainBond.validate(cell)) {
      return null;
    }
    const capacity = BigInt(cell.cell_output.capacity);

    const cellData = new SudtTokenCell(Buffer.from(remove0xPrefix(cell.data), "hex").buffer, { validate: true });

    const sudtAmount = arrayBufferToUint128(cellData.getAmount().raw());

    const lockArgs = new SidechainBondCellLockArgs(scriptArgToArrayBuff(cell.cell_output.lock), { validate: true });

    const participatedChain = readerToChainIdList(lockArgs.getParticipatedChain());
    const collatorLockArg = arrayBufferToPublicKeyHash(lockArgs.getCollatorLockArg().raw());
    const unlockSidechainHeight = arrayBufferToBlockHeight(lockArgs.getUnlockSidechainHeight().raw());

    const outPoint = cell.out_point!;

    return new SidechainBond(capacity, sudtAmount, participatedChain, collatorLockArg, unlockSidechainHeight, outPoint);
  }

  static default(): SidechainBond {
    return new SidechainBond(0n, 0n, [], ``, 0n, defaultOutPoint());
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
    const lock = SIDECHAIN_BOND_LOCK_SCRIPT;

    const sidechainBondCellLockArgs = {
      //convert hex into array buffer
      collator_lock_arg: publicKeyHashToArrayBuffer(this.collatorLockArg),
      participated_chain: chainIdListToWrite(this.participatedChain),
      unlock_sidechain_height: blockHeightToArrayBuffer(this.unlockSidechainHeight),
    };

    const arg = SerializeSidechainBondCellLockArgs(sidechainBondCellLockArgs);

    lock.args = arrayBufferToHex(arg);

    return {
      capacity: Uint64BigIntToLeHex(this.capacity),
      type: SIDECHAIN_BOND_TYPE_SCRIPT,
      lock,
    };
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

  toCellOutputData(): string {
    const sidechainBondCellData = {
      amount: uint128ToArrayBuffer(this.sudtAmount),
    };
    return arrayBufferToHex(SerializeSudtTokenCell(sidechainBondCellData));
  }

  getOutPoint(): string {
    return `${this.outPoint.tx_hash}-${this.outPoint.index}`;
  }

  static fromJSON(source: unknown): SidechainBond {
    return Object.assign(SidechainBond.default(), source);
  }
}
