import {Cell, OutPoint} from '@ckb-lumos/base'
import {
    defaultOutPoint,
    leHexToBigIntUint128,
    leHexToBigIntUint8,
    Uint128BigIntToLeHex,
    Uint64BigIntToLeHex,
} from '../../../utils/tools'
import {CellOutputType} from './interfaces/cell_output_type'
import {CellInputType} from './interfaces/cell_input_type'
import {
    SIDECHAIN_FEE_LOCK_SCRIPT,
    SIDECHAIN_FEE_TYPE_SCRIPT
} from "../../../utils/environment";

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

    capacity: bigint

    museAmount: bigint
    chainId: bigint

    outPoint: OutPoint


    constructor(capacity: bigint, museAmount: bigint, chainId: bigint, outPoint: OutPoint) {
        this.capacity = capacity;
        this.museAmount = museAmount;
        this.chainId = chainId;
        this.outPoint = outPoint;
    }

    static validate(cell: Cell): boolean {
        if (!cell.out_point) {
            return false
        }

        return true
    }

    static fromCell(cell: Cell): SidechainFee | null {
        if (!SidechainFee.validate(cell)) {
            return null
        }
        let capacity = BigInt(cell.cell_output.capacity)

        let museAmount = leHexToBigIntUint128(cell.data)

        let lockArgs = cell.cell_output.lock.args.substring(2)
        let chainId = leHexToBigIntUint8(lockArgs.substring(0, 2))

        let outPoint = cell.out_point!

        return new SidechainFee(capacity, museAmount, chainId, outPoint)
    }

    static default(): SidechainFee {
        return new SidechainFee(0n, 0n, 0n, defaultOutPoint())
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
            type: SIDECHAIN_FEE_TYPE_SCRIPT,
            lock: SIDECHAIN_FEE_LOCK_SCRIPT,
        }
    }

    toCellOutputData(): string {
        return `${Uint128BigIntToLeHex(this.museAmount)}`
    }

    getOutPoint(): string {
        return `${this.outPoint.tx_hash}-${this.outPoint.index}`
    }

    static fromJSON(source: Object): SidechainFee {
        return Object.assign(SidechainFee.default(), source);
    }
}
