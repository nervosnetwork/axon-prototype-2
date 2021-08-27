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
import { CHECKER_INFO_LOCK_SCRIPT, CHECKER_INFO_TYPE_SCRIPT } from "../../../utils/environment";
import {
  CheckerInfoCell,
  CheckerInfoCellTypeArgs,
  SerializeCheckerInfoCell,
  SerializeCheckerInfoCellTypeArgs,
} from "../mol/cellData/checker_info";
import {
  arrayBufferToBytes1,
  arrayBufferToChainId,
  arrayBufferToMolString,
  arrayBufferToPublicKeyHash,
  arrayBufferToUint128,
  bytes1ToArrayBuffer,
  chainIdToArrayBuffer,
  molStringToArrayBuffer,
  publicKeyHashToArrayBuffer,
  uint128ToArrayBuffer,
} from "../../../utils/mol";

/*
checker info

capacity: - 8 bytes
data: - 1+1+16+512+20+1=551
    pub chain_id:                u8,
    pub checker_id:              u8,
    pub unpaid_fee:              u128,
    pub rpc_url:                 [u8; 512],
    pub checker_public_key_hash: [u8; 20],
    pub mode:                    CheckerInfoCellMode,//u8
type: - 54 bytes
    codehash: typeid
    hashtype: type
    args: chain id | public key hash - 21 bytes
lock: - A.S. 33 bytes

 */
export class CheckerInfo implements CellInputType, CellOutputType {
  static RELAYING = "Relaying";
  static QUIT = "Quit";

  outPoint: OutPoint;

  capacity: bigint;

  //data
  unpaidFee: bigint;
  rpcUrl: string;
  status: "Relaying" | "Quit";

  //type args
  chainId: bigint;
  checkerLockArg: string;

  //type args for lumos
  //checkId:bigint
  //checkerPublicKeyHash:string

  constructor(
    capacity: bigint,

    unpaidFee: bigint,
    rpcUrl: string,
    status: "Relaying" | "Quit",

    chainId: bigint,
    checkerLockArg: string,

    outPoint: OutPoint,
  ) {
    this.capacity = capacity;
    this.outPoint = outPoint;

    this.unpaidFee = unpaidFee;
    this.rpcUrl = rpcUrl;
    this.status = status;

    this.chainId = chainId;
    this.checkerLockArg = checkerLockArg;
  }

  static validate(cell: Cell): boolean {
    if (!cell.out_point) {
      return false;
    }

    return true;
  }

  static fromCell(cell: Cell): CheckerInfo | null {
    if (!CheckerInfo.validate(cell)) {
      return null;
    }
    const capacity = BigInt(cell.cell_output.capacity);

    //buf

    const cellData = new CheckerInfoCell(Buffer.from(remove0xPrefix(cell.data), "hex").buffer, { validate: true });

    const unpaidFee = arrayBufferToUint128(cellData.getUnpaidFee().raw());
    const rpcUrl = arrayBufferToMolString(cellData.getRpcUrl().raw());
    const status = arrayBufferToBytes1(cellData.getStatus().raw()) === "0x00" ? "Relaying" : "Quit";

    //args

    if (!cell.cell_output.type) {
      return null;
    }

    const typeArgs = new CheckerInfoCellTypeArgs(scriptArgToArrayBuff(cell.cell_output.type), { validate: true });

    const chainId = arrayBufferToChainId(typeArgs.getChainId().raw());
    const checkerLockArg = arrayBufferToPublicKeyHash(typeArgs.getCheckerLockArg().raw());

    const outPoint = cell.out_point!;

    return new CheckerInfo(capacity, unpaidFee, rpcUrl, status, chainId, checkerLockArg, outPoint);
  }

  static default(): CheckerInfo {
    return new CheckerInfo(0n, 0n, ``, "Relaying", 0n, ``, defaultOutPoint());
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
    const type = CHECKER_INFO_TYPE_SCRIPT;

    const checkerInfoCellTypeArgs = {
      chain_id: chainIdToArrayBuffer(this.chainId),
      checker_lock_arg: publicKeyHashToArrayBuffer(this.checkerLockArg),
    };

    type.args = arrayBufferToHex(SerializeCheckerInfoCellTypeArgs(checkerInfoCellTypeArgs));

    return {
      capacity: Uint64BigIntToLeHex(this.capacity),
      type,
      lock: CHECKER_INFO_LOCK_SCRIPT,
    };
  }

  toCellOutputData(): string {
    const checkerInfoCell = {
      unpaid_fee: uint128ToArrayBuffer(this.unpaidFee),
      rpc_url: molStringToArrayBuffer(this.rpcUrl),
      status: bytes1ToArrayBuffer(this.status == "Relaying" ? "0x00" : "0x01"),
    };

    return arrayBufferToHex(SerializeCheckerInfoCell(checkerInfoCell));
  }

  getOutPoint(): string {
    return `${this.outPoint.tx_hash}-${this.outPoint.index}`;
  }

  static fromJSON(source: unknown): CheckerInfo {
    return Object.assign(CheckerInfo.default(), source);
  }
}
