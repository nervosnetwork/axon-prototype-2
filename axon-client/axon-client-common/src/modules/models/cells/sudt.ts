import { Cell, OutPoint } from "@ckb-lumos/base";
import { arrayBufferToHex, defaultOutPoint, remove0xPrefix, Uint64BigIntToLeHex } from "../../../utils/tools";
import { CellOutputType } from "./interfaces/cell_output_type";
import { CellInputType } from "./interfaces/cell_input_type";
import { SUDT_LOCK_SCRIPT, SUDT_TYPE_SCRIPT } from "../../../utils/environment";
import { Muse } from "./muse";
import { SerializeSudtTokenCell, SudtTokenCell } from "../mol/cellData/sudt_token";
import { arrayBufferToUint128, uint128ToArrayBuffer } from "../../../utils/mol";

/*
muse

capacity: - 8 bytes
data: amount: u128 - 16 bytes
type: - 65 bytes
    code: sudt_type_script
    hashtype: type
    args: muse_owner_lock_hash
lock: - 53 bytes
    codehash: secp256k1 code
    hashtype: type
    args: public key hash - 20 bytes
 */
export class Sudt implements CellInputType, CellOutputType {
  capacity: bigint;
  sudtAmount: bigint;

  outPoint: OutPoint;

  constructor(capacity: bigint, museAmount: bigint, outPoint: OutPoint) {
    this.capacity = capacity;
    this.sudtAmount = museAmount;

    this.outPoint = outPoint;
  }

  static validate(cell: Cell): boolean {
    if (!cell.out_point) {
      return false;
    }

    return true;
  }

  static fromCell(cell: Cell): Sudt | null {
    if (!Sudt.validate(cell)) {
      return null;
    }
    const capacity = BigInt(cell.cell_output.capacity);

    const cellData = new SudtTokenCell(Buffer.from(remove0xPrefix(cell.data), "hex").buffer, { validate: true });

    const sudtAmount = arrayBufferToUint128(cellData.getAmount().raw());

    const outPoint = cell.out_point!;

    return new Sudt(capacity, sudtAmount, outPoint);
  }

  static default(): Sudt {
    return new Sudt(0n, 0n, defaultOutPoint());
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
    return {
      capacity: Uint64BigIntToLeHex(this.capacity),
      type: SUDT_TYPE_SCRIPT,
      lock: SUDT_LOCK_SCRIPT,
    };
  }

  toCellOutputData(): string {
    const sudtCellData = {
      amount: uint128ToArrayBuffer(this.sudtAmount),
    };
    return arrayBufferToHex(SerializeSudtTokenCell(sudtCellData));
  }

  getOutPoint(): string {
    return `${this.outPoint.tx_hash}-${this.outPoint.index}`;
  }

  static fromJSON(source: unknown): Muse {
    return Object.assign(Muse.default(), source);
  }
}
