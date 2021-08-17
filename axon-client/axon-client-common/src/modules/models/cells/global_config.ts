import { Cell, OutPoint } from "@ckb-lumos/base";
import { arrayBufferToHex, defaultOutPoint, remove0xPrefix, Uint64BigIntToLeHex } from "../../../utils/tools";
import { CellOutputType } from "./interfaces/cell_output_type";
import { CellInputType } from "./interfaces/cell_input_type";
import { GLOBAL_CONFIG_LOCK_SCRIPT, GLOBAL_CONFIG_TYPE_SCRIPT } from "../../../utils/environment";
import { CellDepType } from "./interfaces/cell_dep_type";
import { GlobalConfigCell, SerializeGlobalConfigCell } from "../mol/global_config";
import {
  arrayBufferToCodeHash,
  arrayBufferToHashType,
  arrayBufferToPublicKeyHash,
  codeHashToArrayBuffer,
  HASH_TYPE,
  hashTypeToArrayBuffer,
  publicKeyHashToArrayBuffer,
} from "../../../utils/mol";

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
  codeCellTypeHashtype: HASH_TYPE;
  sidechainConfigCellTypeCodehash: string;
  sidechainConfigCellTypeHashtype: HASH_TYPE;
  sidechainStateCellTypeCodehash: string;
  sidechainStateCellTypeHashtype: HASH_TYPE;
  checkerInfoCellTypeCodehash: string;
  checkerInfoCellTypeHashtype: HASH_TYPE;
  checkerBondCellLockCodehash: string;
  checkerBondCellLockHashtype: HASH_TYPE;
  taskCellTypeCodehash: string;
  taskCellTypeHashtype: HASH_TYPE;
  sidechainRegistryCellTypeCodehash: string;
  sidechainRegistryCellTypeHashtype: HASH_TYPE;
  sidechainFeeCellLockCodehash: string;
  sidechainFeeCellLockHashtype: HASH_TYPE;
  sidechainBondCellLockCodehash: string;
  sidechainBondCellLockHashtype: HASH_TYPE;

  outPoint: OutPoint;

  constructor(
    capacity: bigint,
    adminPublicKey: string,
    codeCellTypeCodehash: string,
    codeCellTypeHashtype: HASH_TYPE,
    sidechainConfigCellTypeCodehash: string,
    sidechainConfigCellTypeHashtype: HASH_TYPE,
    sidechain_stateCellTypeCodehash: string,
    sidechain_stateCellTypeHashtype: HASH_TYPE,
    checkerInfoCellTypeCodehash: string,
    checkerInfoCellTypeHashtype: HASH_TYPE,
    checkerBondCellLockCodehash: string,
    checkerBondCellLockHashtype: HASH_TYPE,
    taskCellTypeCodehash: string,
    taskCellTypeHashtype: HASH_TYPE,
    sidechainRegistryCellTypeCodehash: string,
    sidechainRegistryCellTypeHashtype: HASH_TYPE,
    sidechainFeeCellLockCodehash: string,
    sidechainFeeCellLockHashtype: HASH_TYPE,
    sidechainBondCellLockCodehash: string,
    sidechainBondCellLockHashtype: HASH_TYPE,
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
    this.sidechainRegistryCellTypeCodehash = sidechainRegistryCellTypeCodehash;
    this.sidechainRegistryCellTypeHashtype = sidechainRegistryCellTypeHashtype;
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
    const cellData = new GlobalConfigCell(Buffer.from(remove0xPrefix(cell.data), "hex").buffer, { validate: true });

    const adminPublicKey = arrayBufferToPublicKeyHash(cellData.getAdminLockArg().raw());

    const codeCellTypeCodehash = arrayBufferToCodeHash(cellData.getCodeCellTypeCodehash().raw());
    const codeCellTypeHashtype = arrayBufferToHashType(cellData.getCodeCellTypeHashtype().raw());
    const sidechainConfigCellTypeCodehash = arrayBufferToCodeHash(cellData.getSidechainConfigCellTypeCodehash().raw());
    const sidechainConfigCellTypeHashtype = arrayBufferToHashType(cellData.getSidechainConfigCellTypeHashtype().raw());
    const sidechainStateCellTypeCodehash = arrayBufferToCodeHash(cellData.getSidechainStateCellTypeCodehash().raw());
    const sidechainStateCellTypeHashtype = arrayBufferToHashType(cellData.getSidechainStateCellTypeHashtype().raw());
    const checkerInfoCellTypeCodehash = arrayBufferToCodeHash(cellData.getCheckerInfoCellTypeCodehash().raw());
    const checkerInfoCellTypeHashtype = arrayBufferToHashType(cellData.getCheckerInfoCellTypeHashtype().raw());
    const checkerBondCellLockCodehash = arrayBufferToCodeHash(cellData.getCheckerBondCellTypeCodehash().raw());
    const checkerBondCellLockHashtype = arrayBufferToHashType(cellData.getCheckerBondCellTypeHashtype().raw());
    const taskCellTypeCodehash = arrayBufferToCodeHash(cellData.getTaskCellTypeCodehash().raw());
    const taskCellTypeHashtype = arrayBufferToHashType(cellData.getTaskCellTypeHashtype().raw());
    const sidechainRegistryCellTypeCodehash = arrayBufferToCodeHash(
      cellData.getSidechainRegistryCellTypeCodehash().raw(),
    );
    const sidechainRegistryCellTypeHashtype = arrayBufferToHashType(
      cellData.getSidechainRegistryCellTypeHashtype().raw(),
    );
    const sidechainFeeCellLockCodehash = arrayBufferToCodeHash(cellData.getSidechainFeeCellTypeCodehash().raw());
    const sidechainFeeCellLockHashtype = arrayBufferToHashType(cellData.getSidechainFeeCellTypeHashtype().raw());
    const sidechainBondCellLockCodehash = arrayBufferToCodeHash(cellData.getSidechainBondCellTypeCodehash().raw());
    const sidechainBondCellLockHashtype = arrayBufferToHashType(cellData.getSidechainBondCellTypeHashtype().raw());

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
      sidechainRegistryCellTypeCodehash,
      sidechainRegistryCellTypeHashtype,
      sidechainFeeCellLockCodehash,
      sidechainFeeCellLockHashtype,
      sidechainBondCellLockCodehash,
      sidechainBondCellLockHashtype,
      outPoint,
    );
  }

  static default(): GlobalConfig {
    return new GlobalConfig(
      0n,
      ``,
      ``,
      `type`,
      ``,
      `type`,
      ``,
      `type`,
      ``,
      `type`,
      ``,
      `type`,
      ``,
      `type`,
      ``,
      `type`,
      ``,
      `type`,
      ``,
      `type`,
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
    return {
      capacity: Uint64BigIntToLeHex(this.capacity),
      type: GLOBAL_CONFIG_TYPE_SCRIPT,
      lock: GLOBAL_CONFIG_LOCK_SCRIPT,
    };
  }

  toCellOutputData(): string {
    const globalConfigCell = {
      admin_lock_arg: publicKeyHashToArrayBuffer(this.adminPublicKey),

      checker_info_cell_type_codehash: codeHashToArrayBuffer(this.checkerInfoCellTypeCodehash),
      checker_info_cell_type_hashtype: hashTypeToArrayBuffer(this.checkerInfoCellTypeHashtype),

      checker_bond_cell_type_codehash: codeHashToArrayBuffer(this.checkerBondCellLockCodehash),
      checker_bond_cell_type_hashtype: hashTypeToArrayBuffer(this.checkerBondCellLockHashtype),

      code_cell_type_codehash: codeHashToArrayBuffer(this.codeCellTypeCodehash),
      code_cell_type_hashtype: hashTypeToArrayBuffer(this.codeCellTypeHashtype),

      sidechain_bond_cell_type_codehash: codeHashToArrayBuffer(this.sidechainBondCellLockCodehash),
      sidechain_bond_cell_type_hashtype: hashTypeToArrayBuffer(this.sidechainBondCellLockHashtype),

      sidechain_config_cell_type_codehash: codeHashToArrayBuffer(this.sidechainConfigCellTypeCodehash),
      sidechain_config_cell_type_hashtype: hashTypeToArrayBuffer(this.sidechainConfigCellTypeHashtype),

      sidechain_fee_cell_type_codehash: codeHashToArrayBuffer(this.sidechainFeeCellLockCodehash),
      sidechain_fee_cell_type_hashtype: hashTypeToArrayBuffer(this.sidechainFeeCellLockHashtype),

      sidechain_registry_cell_type_codehash: codeHashToArrayBuffer(this.sidechainRegistryCellTypeCodehash),
      sidechain_registry_cell_type_hashtype: hashTypeToArrayBuffer(this.sidechainRegistryCellTypeHashtype),

      sidechain_state_cell_type_codehash: codeHashToArrayBuffer(this.sidechainStateCellTypeCodehash),
      sidechain_state_cell_type_hashtype: hashTypeToArrayBuffer(this.sidechainStateCellTypeHashtype),

      task_cell_type_codehash: codeHashToArrayBuffer(this.taskCellTypeCodehash),
      task_cell_type_hashtype: hashTypeToArrayBuffer(this.taskCellTypeHashtype),
    };

    return arrayBufferToHex(SerializeGlobalConfigCell(globalConfigCell));
  }

  getOutPoint(): string {
    return `${this.outPoint.tx_hash}-${this.outPoint.index}`;
  }

  static fromJSON(source: unknown): GlobalConfig {
    return Object.assign(GlobalConfig.default(), source);
  }
}
