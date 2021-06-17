import {defaultOutPoint} from '../../../utils/tools'
import {CellInputType} from "./interfaces/cell_input_type";
import {OutPoint} from "@ckb-lumos/base";

export class Ckb implements CellInputType {
    static CKB_FIXED_BASE_CAPACITY = BigInt(8 * 10 ** 8)

    // the capacity, which is all the ckb_amount this cell holds
    capacity: bigint = 0n

    outPoint: OutPoint


    constructor(capacity: bigint,  outPoint: OutPoint) {
        this.capacity = capacity
        this.outPoint = outPoint
    }

    static from(capacity: bigint,  outPoint: OutPoint): Ckb {
        return new Ckb(capacity, outPoint)
    }

    static default(): Ckb {
        return new Ckb(0n, defaultOutPoint())
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

    getOutPoint(): string {
        return `${this.outPoint.tx_hash}-${this.outPoint.index}`
    }

}
