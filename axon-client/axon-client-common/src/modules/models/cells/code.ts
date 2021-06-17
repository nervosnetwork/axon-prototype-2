import {Cell, OutPoint} from '@ckb-lumos/base'
import {
    defaultOutPoint,
    leHexToBigIntUint8,
    Uint64BigIntToLeHex,
} from '../../../utils/tools'
import {CellOutputType} from './interfaces/cell_output_type'
import {CellInputType} from './interfaces/cell_input_type'
import {
    CODE_LOCK_SCRIPT,
    CODE_TYPE_SCRIPT
} from "../../../utils/environment";

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


    capacity: bigint

    chainId: bigint
    checkerPublicKeyHash: string

    outPoint: OutPoint


    constructor(capacity: bigint, chainId: bigint, checkerPublicKeyHash: string, outPoint: OutPoint) {
        this.capacity = capacity;
        this.chainId = chainId;
        this.checkerPublicKeyHash = checkerPublicKeyHash;
        this.outPoint = outPoint;
    }

    static validate(cell: Cell): boolean {
        if (!cell.out_point) {
            return false
        }

        return true
    }

    static fromCell(cell: Cell): Code | null {
        if (!Code.validate(cell)) {
            return null
        }
        let capacity = BigInt(cell.cell_output.capacity)

        let lockArgs = cell.cell_output.lock.args.substring(2)

        let chainId = leHexToBigIntUint8(lockArgs.substring(0, 2))
        let checkerPublicKeyHash = lockArgs.substring(2, 42)

        let outPoint = cell.out_point!

        return new Code(capacity, chainId, checkerPublicKeyHash,outPoint)
    }

    static default(): Code {
        return new Code(0n, 0n,  ``, defaultOutPoint())
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
            type: CODE_TYPE_SCRIPT,
            lock: CODE_LOCK_SCRIPT,
        }
    }

    toCellOutputData(): string {
        return `0x`
    }

    getOutPoint(): string {
        return `${this.outPoint.tx_hash}-${this.outPoint.index}`
    }

    static fromJSON(source: Object): Code {
        return Object.assign(Code.default(), source);
    }
}
