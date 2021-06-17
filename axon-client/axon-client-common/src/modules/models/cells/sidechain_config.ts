import {Cell, OutPoint} from '@ckb-lumos/base'
import {
    defaultOutPoint,
    leHexToBigIntUint128,
    leHexToBigIntUint16,
    leHexToBigIntUint8,
    remove0xPrefix,
    Uint128BigIntToLeHex,
    Uint16BigIntToLeHex,
    Uint64BigIntToLeHex,
    Uint8BigIntToLeHex
} from '../../../utils/tools'
import {CellOutputType} from './interfaces/cell_output_type'
import {CellInputType} from './interfaces/cell_input_type'
import {SIDECHAIN_CONFIG_LOCK_SCRIPT, SIDECHAIN_CONFIG_TYPE_SCRIPT} from "../../../utils/environment";
import {CellDepType} from "./interfaces/cell_dep_type";

/*
sidechain config

capacity: - 8 bytes
data: -
    pub chain_id:                u8,
    pub checkerTotalCount:     u8,
    // 2**8 = 256
    pub checkerBitmap:          [u8; 32],
    // 256
    pub checkerThreshold:       u8,
    pub updateInterval:         u16,
    pub minimalBond:            u128,
    pub checkerDataSizeLimit: u128,
    pub checkerPrice:           u128,
    pub refreshInterval:        u16,
    pub commitThreshold:        u8,
    pub challengeThreshold:     u8,
    pub adminPublicKey:        [u8; 32],
    pub collatorPublicKey:     [u8; 32],
    pub bondSudtTypeHash:     [u8; 32],
type: - 65 bytes
    codehash: type id
    hashtype: type
    args: chain id
lock: - A.S. 33 bytes

 */
export class SidechainConfig implements CellInputType, CellOutputType, CellDepType {

    capacity: bigint

    chainId: bigint
    checkerTotalCount: bigint
    checkerBitmap: string
    checkerThreshold: bigint
    updateInterval: bigint
    minimalBond: bigint
    checkerDataSizeLimit: bigint
    checkerPrice: bigint
    refreshInterval: bigint
    commitThreshold: bigint
    challengeThreshold: bigint
    adminPublicKey: string
    collatorPublicKey: string
    bondSudtTypeHash: string

    outPoint: OutPoint


    constructor(capacity: bigint, chainId: bigint, checkerTotalCount: bigint, checkerBitmap: string, checkerThreshold: bigint, updateInterval: bigint, minimalBond: bigint, checkerDataSizeLimit: bigint, checkerPrice: bigint, refreshInterval: bigint, commitThreshold: bigint, challengeThreshold: bigint, adminPublicKey: string, collatorPublicKey: string, bondSudtTypeHash: string, outPoint: OutPoint) {
        this.capacity = capacity;
        this.chainId = chainId;
        this.checkerTotalCount = checkerTotalCount;
        this.checkerBitmap = checkerBitmap;
        this.checkerThreshold = checkerThreshold;
        this.updateInterval = updateInterval;
        this.minimalBond = minimalBond;
        this.checkerDataSizeLimit = checkerDataSizeLimit;
        this.checkerPrice = checkerPrice;
        this.refreshInterval = refreshInterval;
        this.commitThreshold = commitThreshold;
        this.challengeThreshold = challengeThreshold;
        this.adminPublicKey = adminPublicKey;
        this.collatorPublicKey = collatorPublicKey;
        this.bondSudtTypeHash = bondSudtTypeHash;
        this.outPoint = outPoint;
    }

    static validate(cell: Cell): boolean {
        if (!cell.out_point) {
            return false
        }

        return true
    }

    static fromCell(cell: Cell): SidechainConfig | null {
        if (!SidechainConfig.validate(cell)) {
            return null
        }
        let capacity = BigInt(cell.cell_output.capacity)

        let data = cell.data.substring(2)

        let chainId = leHexToBigIntUint8(data.substring(0, 2))
        let checkerTotalCount = leHexToBigIntUint8(data.substring(2, 4))
        let checkerBitmap = data.substring(4, 68)
        let checkerThreshold = leHexToBigIntUint8(data.substring(68, 70))
        let updateInterval = leHexToBigIntUint16(data.substring(70, 74))
        let minimalBond = leHexToBigIntUint128(data.substring(74, 106))
        let checkerDataSizeLimit = leHexToBigIntUint128(data.substring(106, 138))
        let checkerPrice = leHexToBigIntUint128(data.substring(138, 170))
        let refreshInterval = leHexToBigIntUint16(data.substring(170, 174))
        let commitThreshold = leHexToBigIntUint8(data.substring(174, 176))
        let challengeThreshold = leHexToBigIntUint8(data.substring(176, 178))
        let adminPublicKey = data.substring(178, 242)
        let collatorPublicKey = data.substring(242, 306)
        let bondSudtTypeHash = data.substring(306, 370)


        let outPoint = cell.out_point!

        return new SidechainConfig(capacity, chainId, checkerTotalCount, checkerBitmap, checkerThreshold, updateInterval,
            minimalBond, checkerDataSizeLimit, checkerPrice, refreshInterval, commitThreshold, challengeThreshold,
            adminPublicKey, collatorPublicKey, bondSudtTypeHash, outPoint)
    }

    static default(): SidechainConfig {
        return new SidechainConfig(0n, 0n, 0n, ``, 0n, 0n,
            0n, 0n, 0n, 0n, 0n, 0n, ``,
            ``, ``, defaultOutPoint())
    }

    toCellDep(): CKBComponents.CellDep {
        return {
            outPoint:  {
                txHash: this.outPoint.tx_hash,
                index: this.outPoint.index,
            },
            depType:  'code'
        };
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
            type: SIDECHAIN_CONFIG_TYPE_SCRIPT,
            lock: SIDECHAIN_CONFIG_LOCK_SCRIPT,
        }
    }

    toCellOutputData(): string {
        return `0x${
            remove0xPrefix(Uint8BigIntToLeHex(this.chainId))}${
            remove0xPrefix(Uint8BigIntToLeHex(this.checkerTotalCount))}${
            remove0xPrefix(this.checkerBitmap)}${
            remove0xPrefix(Uint128BigIntToLeHex(this.checkerThreshold))}${
            remove0xPrefix(Uint16BigIntToLeHex(this.updateInterval))}${
            remove0xPrefix(Uint128BigIntToLeHex(this.minimalBond))}${
            remove0xPrefix(Uint128BigIntToLeHex(this.checkerDataSizeLimit))}${
            remove0xPrefix(Uint128BigIntToLeHex(this.checkerPrice))}${
            remove0xPrefix(Uint16BigIntToLeHex(this.refreshInterval))}${
            remove0xPrefix(Uint8BigIntToLeHex(this.commitThreshold))}${
            remove0xPrefix(Uint8BigIntToLeHex(this.challengeThreshold))}${
            remove0xPrefix(this.adminPublicKey)}${
            remove0xPrefix(this.collatorPublicKey)}${
            remove0xPrefix(this.bondSudtTypeHash)}`
    }

    getOutPoint(): string {
        return `${this.outPoint.tx_hash}-${this.outPoint.index}`
    }

    static fromJSON(source: Object): SidechainConfig {
        return Object.assign(SidechainConfig.default(), source);
    }
}
