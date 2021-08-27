import { Cell, OutPoint } from "@ckb-lumos/base";
import { arrayBufferToHex, defaultOutPoint, remove0xPrefix, Uint64BigIntToLeHex } from "../../../utils/tools";
import { CellOutputType } from "./interfaces/cell_output_type";
import { CellInputType } from "./interfaces/cell_input_type";
import { MUSE_LOCK_SCRIPT, MUSE_TYPE_SCRIPT } from "../../../utils/environment";
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
export class Muse implements CellInputType, CellOutputType {
  capacity: bigint;
  museAmount: bigint;

  outPoint: OutPoint;

  constructor(capacity: bigint, museAmount: bigint, outPoint: OutPoint) {
    this.capacity = capacity;
    this.museAmount = museAmount;

    this.outPoint = outPoint;
  }

  static validate(cell: Cell): boolean {
    if (!cell.out_point) {
      return false;
    }

    return true;
  }

  static fromCell(cell: Cell): Muse | null {
    if (!Muse.validate(cell)) {
      return null;
    }
    const capacity = BigInt(cell.cell_output.capacity);

    const cellData = new SudtTokenCell(Buffer.from(remove0xPrefix(cell.data), "hex").buffer, { validate: true });

    const museAmount = arrayBufferToUint128(cellData.getAmount().raw());

    const outPoint = cell.out_point!;

    return new Muse(capacity, museAmount, outPoint);
  }

  static default(): Muse {
    return new Muse(0n, 0n, defaultOutPoint());
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
      type: MUSE_TYPE_SCRIPT,
      lock: MUSE_LOCK_SCRIPT,
    };
  }

  toCellOutputData(): string {
    const museCellData = {
      amount: uint128ToArrayBuffer(this.museAmount),
    };
    return arrayBufferToHex(SerializeSudtTokenCell(museCellData));
  }

  getOutPoint(): string {
    return `${this.outPoint.tx_hash}-${this.outPoint.index}`;
  }

  static fromJSON(source: unknown): Muse {
    return Object.assign(Muse.default(), source);
  }
}
