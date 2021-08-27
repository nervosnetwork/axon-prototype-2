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
import { CHECKER_BOND_LOCK_SCRIPT, CHECKER_BOND_TYPE_SCRIPT } from "../../../utils/environment";
import { SerializeSudtTokenCell, SudtTokenCell } from "../mol/cellData/sudt_token";
import { CheckerBondCellLockArgs, SerializeCheckerBondCellLockArgs } from "../mol/cellData/checker_bond";
import {
  arrayBufferToPublicKeyHash,
  arrayBufferToUint128,
  chainIdListToWrite,
  publicKeyHashToArrayBuffer,
  readerToChainIdList,
  uint128ToArrayBuffer,
} from "../../../utils/mol";

/*
checker bond

capacity: - 8 bytes
data: amount: u128 - 16 bytes
type: - 65 bytes
    codehash: sudt_type_script
    hashtype: type
    args: muse_owner_lock_hash
lock: - 85 bytes
    codehash: typeid
    hashtype: type
    args: checker public key hash | chain id bitmap - 52 bytes
 */
export class CheckerBond implements CellInputType, CellOutputType {
  outPoint: OutPoint;

  capacity: bigint;

  //data
  museAmount: bigint;

  //lock args
  checkerLockArg: string;
  participatedChainId: Array<bigint>;

  constructor(
    capacity: bigint,
    museAmount: bigint,
    checkerLockArg: string,
    participatedChainId: Array<bigint>,
    outPoint: OutPoint,
  ) {
    this.capacity = capacity;
    this.museAmount = museAmount;
    this.checkerLockArg = checkerLockArg;
    this.participatedChainId = participatedChainId;

    this.outPoint = outPoint;
  }

  static validate(cell: Cell): boolean {
    if (!cell.out_point) {
      return false;
    }

    return true;
  }

  static fromCell(cell: Cell): CheckerBond | null {
    if (!CheckerBond.validate(cell)) {
      return null;
    }
    const capacity = BigInt(cell.cell_output.capacity);

    const cellData = new SudtTokenCell(Buffer.from(remove0xPrefix(cell.data), "hex").buffer, { validate: true });

    const sudtAmount = arrayBufferToUint128(cellData.getAmount().raw());

    //==============================================================================

    const lockArgs = new CheckerBondCellLockArgs(scriptArgToArrayBuff(cell.cell_output.lock), { validate: true });

    const checkerLockArg = arrayBufferToPublicKeyHash(lockArgs.getCheckerLockArg().raw());
    const participated_chain_id = readerToChainIdList(lockArgs.getParticipatedChainId());

    const outPoint = cell.out_point!;

    return new CheckerBond(capacity, sudtAmount, checkerLockArg, participated_chain_id, outPoint);
  }

  static default(): CheckerBond {
    return new CheckerBond(0n, 0n, ``, [], defaultOutPoint());
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
    const lock = CHECKER_BOND_LOCK_SCRIPT;

    const checkerBondCellLockArgs = {
      //convert hex into array buffer
      checker_lock_arg: publicKeyHashToArrayBuffer(this.checkerLockArg),
      participated_chain_id: chainIdListToWrite(this.participatedChainId),
    };

    const arg = SerializeCheckerBondCellLockArgs(checkerBondCellLockArgs);

    lock.args = arrayBufferToHex(arg);

    return {
      capacity: Uint64BigIntToLeHex(this.capacity),
      type: CHECKER_BOND_TYPE_SCRIPT,
      lock,
    };
  }

  toCellOutputData(): string {
    const checkerBondCell = {
      amount: uint128ToArrayBuffer(this.museAmount),
    };
    return arrayBufferToHex(SerializeSudtTokenCell(checkerBondCell));
  }

  getOutPoint(): string {
    return `${this.outPoint.tx_hash}-${this.outPoint.index}`;
  }

  static fromJSON(source: unknown): CheckerBond {
    return Object.assign(CheckerBond.default(), source);
  }
}
