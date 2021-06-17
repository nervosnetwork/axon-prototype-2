import {Cell, OutPoint} from '@ckb-lumos/base'
import {
    defaultOutPoint,
    leHexToBigIntUint128,
    leHexToBigIntUint8,
    remove0xPrefix,
    Uint128BigIntToLeHex,
    Uint64BigIntToLeHex,
    Uint8BigIntToLeHex
} from '../../../utils/tools'
import {CellOutputType} from './interfaces/cell_output_type'
import {CellInputType} from './interfaces/cell_input_type'
import {CHECKER_INFO_LOCK_SCRIPT, CHECKER_INFO_TYPE_SCRIPT} from "../../../utils/environment";

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


    static CHECKER_IDLE = 0n
    static TASK_PASSED = 1n
    static CHALLENGE_PASSED = 2n
    static CHALLENGE_REJECTED = 3n


    capacity: bigint

    chainId: bigint
    checkId: bigint
    unpaidFee: bigint
    rpcUrl: string
    checkerPublicKeyHash: string
    mode: bigint

    outPoint: OutPoint

    //type args for lumos
    //checkId:bigint
    //checkerPublicKeyHash:string


    constructor(capacity: bigint, chainId: bigint, checkId: bigint, unpaidFee: bigint, rpcUrl: string, checkerPublicKeyHash: string, mode: bigint, outPoint: OutPoint) {
        this.capacity = capacity;
        this.chainId = chainId;
        this.checkId = checkId;
        this.unpaidFee = unpaidFee;
        this.rpcUrl = rpcUrl;
        this.checkerPublicKeyHash = checkerPublicKeyHash;
        this.mode = mode;
        this.outPoint = outPoint;
    }

    static validate(cell: Cell): boolean {
        if (!cell.out_point) {
            return false
        }

        return true
    }

    static fromCell(cell: Cell): CheckerInfo | null {
        if (!CheckerInfo.validate(cell)) {
            return null
        }
        let capacity = BigInt(cell.cell_output.capacity)

        let data = cell.data.substring(2)

        let chainId = leHexToBigIntUint8(data.substring(0, 2))
        let checkId = leHexToBigIntUint8(data.substring(2, 4))
        let unpaidFee = leHexToBigIntUint128(data.substring(4, 68))
        let rpcUrl = data.substring(68, 1092);
        let checkerPublicKeyHash = data.substring(1092, 1132);
        let mode = leHexToBigIntUint8(data.substring(1132, 1134));


        let outPoint = cell.out_point!

        return new CheckerInfo(capacity, chainId, checkId, unpaidFee, rpcUrl, checkerPublicKeyHash, mode, outPoint)
    }

    static default(): CheckerInfo {
        return new CheckerInfo(0n, 0n, 0n, 0n, ``, ``, 0n, defaultOutPoint())
    }

    toCellInput(): CKBComponents.CellInput {
        return {
            previousOutput: {
                txHash: this.outPoint.tx_hash,
                index: this.outPoint.index,
            },
            since: '0x0',
        }
    }

    toCellOutput(): CKBComponents.CellOutput {

        let type = CHECKER_INFO_TYPE_SCRIPT
        type.args = `0x${
            remove0xPrefix(Uint8BigIntToLeHex(this.chainId))}${
            remove0xPrefix(this.checkerPublicKeyHash)}`

        return {
            capacity: Uint64BigIntToLeHex(this.capacity),
            type,
            lock: CHECKER_INFO_LOCK_SCRIPT,
        }
    }

    toCellOutputData(): string {
        return `${
            Uint8BigIntToLeHex(this.chainId)}${
            remove0xPrefix(Uint8BigIntToLeHex(this.checkId))}${
            remove0xPrefix(Uint128BigIntToLeHex(this.unpaidFee))}${
            remove0xPrefix(this.rpcUrl)}${
            remove0xPrefix(this.checkerPublicKeyHash)}${
            remove0xPrefix(Uint8BigIntToLeHex(this.mode))}`
    }

    getOutPoint(): string {
        return `${this.outPoint.tx_hash}-${this.outPoint.index}`
    }

    static fromJSON(source: Object): CheckerInfo {
        return Object.assign(CheckerInfo.default(), source);
    }
}
