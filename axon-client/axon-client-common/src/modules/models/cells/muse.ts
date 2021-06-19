import {Cell, OutPoint} from '@ckb-lumos/base'
import {defaultOutPoint, leHexToBigIntUint128, Uint128BigIntToLeHex, Uint64BigIntToLeHex} from '../../../utils/tools'
import {CellOutputType} from './interfaces/cell_output_type'
import {CellInputType} from './interfaces/cell_input_type'
import {MUSE_LOCK_SCRIPT, MUSE_TYPE_SCRIPT} from "../../../utils/environment";

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

    capacity: bigint
    museAmount: bigint



    outPoint: OutPoint

    constructor(capacity: bigint, museAmount: bigint, outPoint: OutPoint) {
        this.capacity = capacity
        this.museAmount = museAmount

        this.outPoint = outPoint
    }

    static validate(cell: Cell): boolean {
        if (!cell.out_point) {
            return false
        }

        return true
    }

    static fromCell(cell: Cell): Muse | null {
        if (!Muse.validate(cell)) {
            return null
        }
        let capacity = BigInt(cell.cell_output.capacity)
        let museAmount = leHexToBigIntUint128(cell.data)

        let outPoint = cell.out_point!

        return new Muse(capacity, museAmount, outPoint)
    }

    static default(): Muse {
        return new Muse(0n, 0n, defaultOutPoint())
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
        return {
            capacity: Uint64BigIntToLeHex(this.capacity),
            type: MUSE_TYPE_SCRIPT,
            lock: MUSE_LOCK_SCRIPT,
        }
    }

    toCellOutputData(): string {
        return `${Uint128BigIntToLeHex(this.museAmount)}`
    }

    getOutPoint(): string {
        return `${this.outPoint.tx_hash}-${this.outPoint.index}`
    }

    static fromJSON(source: Object): Muse {
        return Object.assign(Muse.default(), source);
    }
}
