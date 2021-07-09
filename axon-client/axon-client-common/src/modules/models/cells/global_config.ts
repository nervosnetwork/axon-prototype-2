import { Cell, OutPoint } from "@ckb-lumos/base";
import {
  defaultOutPoint,
  leHexToBigIntUint8,
  remove0xPrefix,
  Uint64BigIntToLeHex,
  Uint8BigIntToLeHex,
} from "../../../utils/tools";
import { CellOutputType } from "./interfaces/cell_output_type";
import { CellInputType } from "./interfaces/cell_input_type";
import { GLOBAL_CONFIG_LOCK_SCRIPT, GLOBAL_CONFIG_TYPE_SCRIPT } from "../../../utils/environment";
import { CellDepType } from "./interfaces/cell_dep_type";

/**

 Global config cell only contains data

 Global Config Cell
 Data:
 Type:
 codehash: typeid                // A.S.
 hashtype: type                  // data
 args: unique_id                 // null
 Lock:
 codehash: secp256k1
 args: admin
 */
export class GlobalConfig implements CellInputType, CellOutputType, CellDepType {
  capacity: bigint;

  adminPublicKey: string;
  codeCellTypeCodehash: string;
  codeCellTypeHashtype: bigint;
  sidechainConfigCellTypeCodehash: string;
  sidechainConfigCellTypeHashtype: bigint;
  sidechainStateCellTypeCodehash: string;
  sidechainStateCellTypeHashtype: bigint;
  checkerInfoCellTypeCodehash: string;
  checkerInfoCellTypeHashtype: bigint;
  checkerBondCellLockCodehash: string;
  checkerBondCellLockHashtype: bigint;
  taskCellTypeCodehash: string;
  taskCellTypeHashtype: bigint;
  sidechainFeeCellLockCodehash: string;
  sidechainFeeCellLockHashtype: bigint;
  sidechainBondCellLockCodehash: string;
  sidechainBondCellLockHashtype: bigint;

  outPoint: OutPoint;

  constructor(
    capacity: bigint,
    adminPublicKey: string,
    codeCellTypeCodehash: string,
    codeCellTypeHashtype: bigint,
    sidechainConfigCellTypeCodehash: string,
    sidechainConfigCellTypeHashtype: bigint,
    sidechain_stateCellTypeCodehash: string,
    sidechain_stateCellTypeHashtype: bigint,
    checkerInfoCellTypeCodehash: string,
    checkerInfoCellTypeHashtype: bigint,
    checkerBondCellLockCodehash: string,
    checkerBondCellLockHashtype: bigint,
    taskCellTypeCodehash: string,
    taskCellTypeHashtype: bigint,
    sidechainFeeCellLockCodehash: string,
    sidechainFeeCellLockHashtype: bigint,
    sidechainBondCellLockCodehash: string,
    sidechainBondCellLockHashtype: bigint,
    outPoint: OutPoint,
  ) {
    this.capacity = capacity;
    this.adminPublicKey = adminPublicKey;
    this.codeCellTypeCodehash = codeCellTypeCodehash;
    this.codeCellTypeHashtype = codeCellTypeHashtype;
    this.sidechainConfigCellTypeCodehash = sidechainConfigCellTypeCodehash;
    this.sidechainConfigCellTypeHashtype = sidechainConfigCellTypeHashtype;
    this.sidechainStateCellTypeCodehash = sidechain_stateCellTypeCodehash;
    this.sidechainStateCellTypeHashtype = sidechain_stateCellTypeHashtype;
    this.checkerInfoCellTypeCodehash = checkerInfoCellTypeCodehash;
    this.checkerInfoCellTypeHashtype = checkerInfoCellTypeHashtype;
    this.checkerBondCellLockCodehash = checkerBondCellLockCodehash;
    this.checkerBondCellLockHashtype = checkerBondCellLockHashtype;
    this.taskCellTypeCodehash = taskCellTypeCodehash;
    this.taskCellTypeHashtype = taskCellTypeHashtype;
    this.sidechainFeeCellLockCodehash = sidechainFeeCellLockCodehash;
    this.sidechainFeeCellLockHashtype = sidechainFeeCellLockHashtype;
    this.sidechainBondCellLockCodehash = sidechainBondCellLockCodehash;
    this.sidechainBondCellLockHashtype = sidechainBondCellLockHashtype;
    this.outPoint = outPoint;
  }

  static validate(cell: Cell): boolean {
    if (!cell.out_point) {
      return false;
    }

    return true;
  }

  static fromCell(cell: Cell): GlobalConfig | null {
    if (!GlobalConfig.validate(cell)) {
      return null;
    }
    const capacity = BigInt(cell.cell_output.capacity);

    const data = cell.data.substring(2);

    const adminPublicKey = data.substring(0, 64);

    const codeCellTypeCodehash = data.substring(0, 64);
    const codeCellTypeHashtype = leHexToBigIntUint8(data.substring(64, 128));
    const sidechainConfigCellTypeCodehash = data.substring(128, 130);
    const sidechainConfigCellTypeHashtype = leHexToBigIntUint8(data.substring(130, 194));
    const sidechainStateCellTypeCodehash = data.substring(194, 196);
    const sidechainStateCellTypeHashtype = leHexToBigIntUint8(data.substring(196, 198));
    const checkerInfoCellTypeCodehash = data.substring(198, 262);
    const checkerInfoCellTypeHashtype = leHexToBigIntUint8(data.substring(262, 264));
    const checkerBondCellLockCodehash = data.substring(264, 328);
    const checkerBondCellLockHashtype = leHexToBigIntUint8(data.substring(328, 330));
    const taskCellTypeCodehash = data.substring(330, 394);
    const taskCellTypeHashtype = leHexToBigIntUint8(data.substring(394, 396));
    const sidechainFeeCellLockCodehash = data.substring(396, 460);
    const sidechainFeeCellLockHashtype = leHexToBigIntUint8(data.substring(460, 462));
    const sidechainBondCellLockCodehash = data.substring(462, 526);
    const sidechainBondCellLockHashtype = leHexToBigIntUint8(data.substring(526, 528));

    const outPoint = cell.out_point!;

    return new GlobalConfig(
      capacity,
      adminPublicKey,
      codeCellTypeCodehash,
      codeCellTypeHashtype,
      sidechainConfigCellTypeCodehash,
      sidechainConfigCellTypeHashtype,
      sidechainStateCellTypeCodehash,
      sidechainStateCellTypeHashtype,
      checkerInfoCellTypeCodehash,
      checkerInfoCellTypeHashtype,
      checkerBondCellLockCodehash,
      checkerBondCellLockHashtype,
      taskCellTypeCodehash,
      taskCellTypeHashtype,
      sidechainFeeCellLockCodehash,
      sidechainFeeCellLockHashtype,
      sidechainBondCellLockCodehash,
      sidechainBondCellLockHashtype,
      outPoint,
    );
  }

  static default(): GlobalConfig {
    return new GlobalConfig(0n, ``, ``, 0n, ``, 0n, ``, 0n, ``, 0n, ``, 0n, ``, 0n, ``, 0n, ``, 0n, defaultOutPoint());
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
    return {
      capacity: Uint64BigIntToLeHex(this.capacity),
      type: GLOBAL_CONFIG_TYPE_SCRIPT,
      lock: GLOBAL_CONFIG_LOCK_SCRIPT,
    };
  }

  toCellOutputData(): string {
    return `0x${remove0xPrefix(this.adminPublicKey)}${remove0xPrefix(this.codeCellTypeCodehash)}${remove0xPrefix(
      Uint8BigIntToLeHex(this.codeCellTypeHashtype),
    )}${remove0xPrefix(this.sidechainConfigCellTypeCodehash)}${remove0xPrefix(
      Uint8BigIntToLeHex(this.sidechainConfigCellTypeHashtype),
    )}${remove0xPrefix(this.sidechainStateCellTypeCodehash)}${remove0xPrefix(
      Uint8BigIntToLeHex(this.sidechainStateCellTypeHashtype),
    )}${remove0xPrefix(this.checkerInfoCellTypeCodehash)}${remove0xPrefix(
      Uint8BigIntToLeHex(this.checkerInfoCellTypeHashtype),
    )}${remove0xPrefix(this.checkerBondCellLockCodehash)}${remove0xPrefix(
      Uint8BigIntToLeHex(this.checkerBondCellLockHashtype),
    )}${remove0xPrefix(this.taskCellTypeCodehash)}${remove0xPrefix(
      Uint8BigIntToLeHex(this.taskCellTypeHashtype),
    )}${remove0xPrefix(this.sidechainFeeCellLockCodehash)}${remove0xPrefix(
      Uint8BigIntToLeHex(this.sidechainFeeCellLockHashtype),
    )}${remove0xPrefix(this.sidechainBondCellLockCodehash)}${remove0xPrefix(
      Uint8BigIntToLeHex(this.sidechainBondCellLockHashtype),
    )}`;
  }

  getOutPoint(): string {
    return `${this.outPoint.tx_hash}-${this.outPoint.index}`;
  }

  static fromJSON(source: unknown): GlobalConfig {
    return Object.assign(GlobalConfig.default(), source);
  }
}
