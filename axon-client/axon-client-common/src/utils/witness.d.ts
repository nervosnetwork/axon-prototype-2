export interface CastToArrayBuffer {
    toArrayBuffer(): ArrayBuffer;
}

export type CanCastToArrayBuffer = ArrayBuffer | CastToArrayBuffer;

export interface CreateOptions {
    validate?: boolean;
}

export interface UnionType {
    type: string;
    value: any;
}

export function SerializeMintTokenWitness(value: object): ArrayBuffer;

export class MintTokenWitness {
    constructor(reader: CanCastToArrayBuffer, options?: CreateOptions);

    validate(compatible?: boolean): void;

    getMode(): number;

    getSpvProof(): Bytes;

    getCellDepIndexList(): Bytes;

    getMerkleProof(): Bytes;
}

export function SerializeETHSPVProof(value: object): ArrayBuffer;

export class ETHSPVProof {
    constructor(reader: CanCastToArrayBuffer, options?: CreateOptions);

    validate(compatible?: boolean): void;

    getLogIndex(): Uint64;

    getReceiptIndex(): Uint64;

    getReceiptData(): Bytes;

    getHeaderData(): Bytes;

    getProof(): BytesVec;
}

export function SerializeBytes(value: CanCastToArrayBuffer): ArrayBuffer;

export class Bytes {
    constructor(reader: CanCastToArrayBuffer, options?: CreateOptions);

    validate(compatible?: boolean): void;

    indexAt(i: number): number;

    raw(): ArrayBuffer;

    length(): number;
}

export function SerializeByte32(value: CanCastToArrayBuffer): ArrayBuffer;

export class Byte32 {
    constructor(reader: CanCastToArrayBuffer, options?: CreateOptions);

    validate(compatible?: boolean): void;

    indexAt(i: number): number;

    raw(): ArrayBuffer;

    static size(): Number;
}

export function SerializeETHAddress(value: CanCastToArrayBuffer): ArrayBuffer;

export class ETHAddress {
    constructor(reader: CanCastToArrayBuffer, options?: CreateOptions);

    validate(compatible?: boolean): void;

    indexAt(i: number): number;

    raw(): ArrayBuffer;

    static size(): Number;
}

export function SerializeBytesVec(value: Array<CanCastToArrayBuffer>): ArrayBuffer;

export class BytesVec {
    constructor(reader: CanCastToArrayBuffer, options?: CreateOptions);

    validate(compatible?: boolean): void;

    indexAt(i: number): Bytes;

    length(): number;
}

export function SerializeUint64(value: CanCastToArrayBuffer): ArrayBuffer;

export class Uint64 {
    constructor(reader: CanCastToArrayBuffer, options?: CreateOptions);

    validate(compatible?: boolean): void;

    indexAt(i: number): number;

    raw(): ArrayBuffer;

    static size(): Number;
}

export function SerializeUint128(value: CanCastToArrayBuffer): ArrayBuffer;

export class Uint128 {
    constructor(reader: CanCastToArrayBuffer, options?: CreateOptions);

    validate(compatible?: boolean): void;

    indexAt(i: number): number;

    raw(): ArrayBuffer;

    static size(): Number;
}


