import {Cell, OutPoint} from '@ckb-lumos/base'
import {
    defaultOutPoint,
    leHexToBigIntUint128,
    remove0xPrefix,
    Uint128BigIntToLeHex,
    Uint64BigIntToLeHex
} from '../../../utils/tools'
import {CellOutputType} from './interfaces/cell_output_type'
import {CellInputType} from './interfaces/cell_input_type'
import {
    CHECKER_BOND_LOCK_SCRIPT,
    CHECKER_BOND_TYPE_SCRIPT,
} from "../../../utils/environment";

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

    capacity: bigint
    museAmount: bigint

    checkerPublicKeyHash: string
    chainIdBitmap: string

    outPoint: OutPoint

    constructor(capacity: bigint, sudtAmount: bigint, checkerPublicKeyHash: string, chainIdBitmap: string, outPoint: OutPoint) {
        this.capacity = capacity
        this.museAmount = sudtAmount
        this.checkerPublicKeyHash = checkerPublicKeyHash
        this.chainIdBitmap = chainIdBitmap

        this.outPoint = outPoint
    }

    static validate(cell: Cell): boolean {
        if (!cell.out_point) {
            return false
        }

        return true
    }

    static fromCell(cell: Cell): CheckerBond | null {
        if (!CheckerBond.validate(cell)) {
            return null
        }
        let capacity = BigInt(cell.cell_output.capacity)
        let sudtAmount = leHexToBigIntUint128(cell.data)

        let lockArgs = cell.cell_output.lock.args.substring(2)

        let checkerPublicKeyHash = lockArgs.substring(0, 40)
        let chainIdBitmap = lockArgs.substring(40, 104)

        let outPoint = cell.out_point!

        return new CheckerBond(capacity, sudtAmount, checkerPublicKeyHash, chainIdBitmap, outPoint)
    }

    static default(): CheckerBond {
        return new CheckerBond(0n, 0n, ``,``,defaultOutPoint())
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
        let lock = CHECKER_BOND_LOCK_SCRIPT
        lock.args = `0x${
            remove0xPrefix(this.checkerPublicKeyHash)}${
            remove0xPrefix(this.chainIdBitmap)}`
        return {
            capacity: Uint64BigIntToLeHex(this.capacity),
            type: CHECKER_BOND_TYPE_SCRIPT,
            lock,
        }
    }

    toCellOutputData(): string {
        return `${Uint128BigIntToLeHex(this.museAmount)}`
    }

    getOutPoint(): string {
        return `${this.outPoint.tx_hash}-${this.outPoint.index}`
    }

    static fromJSON(source: Object): CheckerBond {
        return Object.assign(CheckerBond.default(), source);
    }
}
