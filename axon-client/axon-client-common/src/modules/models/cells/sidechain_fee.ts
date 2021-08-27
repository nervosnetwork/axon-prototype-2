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
import { SIDECHAIN_FEE_LOCK_SCRIPT, SIDECHAIN_FEE_TYPE_SCRIPT } from "../../../utils/environment";
import { SerializeSudtTokenCell, SudtTokenCell } from "../mol/cellData/sudt_token";
import { arrayBufferToPublicKeyHash, arrayBufferToUint128, uint128ToArrayBuffer } from "../../../utils/mol";
import { SidechainFeeCellLockArgs } from "../mol/cellData/sidechain_fee";

/*
sidechain fee

capacity: - 8 bytes
data: amount: u128 - 16 bytes
type: - 65 bytes
    codehash: sudt_type_script
    hashtype: type
    args: muse_owner_lock_hash
lock:
    codehash: typeid
    hashtype: type
    args: chain id
 */
export class SidechainFee implements CellInputType, CellOutputType {
  capacity: bigint;

  museAmount: bigint;
  chainId: string;

  outPoint: OutPoint;

  constructor(capacity: bigint, museAmount: bigint, chainId: string, outPoint: OutPoint) {
    this.capacity = capacity;
    this.museAmount = museAmount;
    this.chainId = chainId;
    this.outPoint = outPoint;
  }

  static validate(cell: Cell): boolean {
    if (!cell.out_point) {
      return false;
    }

    return true;
  }

  static fromCell(cell: Cell): SidechainFee | null {
    if (!SidechainFee.validate(cell)) {
      return null;
    }
    const capacity = BigInt(cell.cell_output.capacity);

    const cellData = new SudtTokenCell(Buffer.from(remove0xPrefix(cell.data), "hex").buffer, { validate: true });

    const museAmount = arrayBufferToUint128(cellData.getAmount().raw());

    const lockArgs = new SidechainFeeCellLockArgs(scriptArgToArrayBuff(cell.cell_output.lock), { validate: true });

    const chainId = arrayBufferToPublicKeyHash(lockArgs.getChainId().raw());

    const outPoint = cell.out_point!;

    return new SidechainFee(capacity, museAmount, chainId, outPoint);
  }

  static default(): SidechainFee {
    return new SidechainFee(0n, 0n, ``, defaultOutPoint());
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
      type: SIDECHAIN_FEE_TYPE_SCRIPT,
      lock: SIDECHAIN_FEE_LOCK_SCRIPT,
    };
  }

  toCellOutputData(): string {
    const sidechainFeeCell = {
      amount: uint128ToArrayBuffer(this.museAmount),
    };
    return arrayBufferToHex(SerializeSudtTokenCell(sidechainFeeCell));
  }

  getOutPoint(): string {
    return `${this.outPoint.tx_hash}-${this.outPoint.index}`;
  }

  static fromJSON(source: unknown): SidechainFee {
    return Object.assign(SidechainFee.default(), source);
  }
}
