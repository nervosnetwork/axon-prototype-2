import { Cell, OutPoint } from "@ckb-lumos/base";
import { defaultOutPoint, scriptArgToArrayBuff, bigIntToHex } from "../../../utils/tools";
import { CellOutputType } from "./interfaces/cell_output_type";
import { CellInputType } from "./interfaces/cell_input_type";
import { CODE_LOCK_SCRIPT, CODE_TYPE_SCRIPT } from "../../../utils/environment";
import { CodeCellLockArgs } from "../mol/cellData/code";
import { arrayBufferToPublicKeyHash } from "../../../utils/mol";

import assert from "assert";

/*
code

capacity: - 8 bytes
data: - null

type: - 65 bytes
    codehash: typeid
    hashtype: type
    args: chain id | public key hash - 21 bytes
lock: - 65 bytes
    codehash: secp256k1
    hashtype: type
    args: owner public key hash - 20 bytes

 */
export class Code implements CellInputType, CellOutputType {
  capacity: bigint;

  // lock args
  lockArg: string;

  outPoint?: OutPoint;

  constructor(capacity: bigint, lockArg: string, outPoint?: OutPoint) {
    this.capacity = capacity;
    this.lockArg = lockArg;
    this.outPoint = outPoint;
  }

  static validate(cell: Cell): boolean {
    if (!cell.out_point) {
      return false;
    }

    return true;
  }

  static fromCell(cell: Cell): Code | null {
    if (!Code.validate(cell)) {
      return null;
    }
    const capacity = BigInt(cell.cell_output.capacity);

    const lockArgs = new CodeCellLockArgs(scriptArgToArrayBuff(cell.cell_output.lock), { validate: true });

    const lockArg = arrayBufferToPublicKeyHash(lockArgs.getLockArg().raw());

    const outPoint = cell.out_point;
    assert(outPoint);

    return new Code(capacity, lockArg, outPoint);
  }

  static default(): Code {
    return new Code(0n, "", defaultOutPoint());
  }

  toCellInput(): CKBComponents.CellInput {
    assert(this.outPoint);

    return {
      previousOutput: {
        txHash: this.outPoint.tx_hash,
        index: this.outPoint.index,
      },
      since: "0x0",
    };
  }

  toCellOutput(): CKBComponents.CellOutput {
    //skip refresh reset lock args
    return {
      capacity: bigIntToHex(this.capacity),
      type: CODE_TYPE_SCRIPT,
      lock: {
        ...CODE_LOCK_SCRIPT,
        args: this.lockArg,
      },
    };
  }

  toCellOutputData(): string {
    return `0x`;
  }

  getOutPoint(): string {
    assert(this.outPoint);
    return `${this.outPoint.tx_hash}-${this.outPoint.index}`;
  }

  static fromJSON(source: unknown): Code {
    return Object.assign(Code.default(), source);
  }
}
