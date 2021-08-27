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

export function SerializeJobs(value: Array<object>): ArrayBuffer;
export class Jobs {
  constructor(reader: CanCastToArrayBuffer, options?: CreateOptions);
  validate(compatible?: boolean): void;
  indexAt(i: number): BlockSlice;
  length(): number;
}

export function SerializeCommittedCheckerInfo(value: object): ArrayBuffer;
export class CommittedCheckerInfo {
  constructor(reader: CanCastToArrayBuffer, options?: CreateOptions);
  validate(compatible?: boolean): void;
  static size(): Number;
  getCheckerLockArg(): PubKeyHash;
  getCommittedHash(): CommittedHash;
}

export function SerializeCommittedCheckerInfos(value: Array<object>): ArrayBuffer;
export class CommittedCheckerInfos {
  constructor(reader: CanCastToArrayBuffer, options?: CreateOptions);
  validate(compatible?: boolean): void;
  indexAt(i: number): CommittedCheckerInfo;
  length(): number;
}

export function SerializePunishedChecker(value: object): ArrayBuffer;
export class PunishedChecker {
  constructor(reader: CanCastToArrayBuffer, options?: CreateOptions);
  validate(compatible?: boolean): void;
  static size(): Number;
  getCheckerLockArg(): PubKeyHash;
  getPunishPoints(): Uint32;
}

export function SerializePunishedCheckers(value: Array<object>): ArrayBuffer;
export class PunishedCheckers {
  constructor(reader: CanCastToArrayBuffer, options?: CreateOptions);
  validate(compatible?: boolean): void;
  indexAt(i: number): PunishedChecker;
  length(): number;
}

export function SerializeBlockHeaders(value: Array<CanCastToArrayBuffer>): ArrayBuffer;
export class BlockHeaders {
  constructor(reader: CanCastToArrayBuffer, options?: CreateOptions);
  validate(compatible?: boolean): void;
  indexAt(i: number): BlockHeader;
  length(): number;
}

export function SerializeCheckerLastAcceptTaskHeight(value: object): ArrayBuffer;
export class CheckerLastAcceptTaskHeight {
  constructor(reader: CanCastToArrayBuffer, options?: CreateOptions);
  validate(compatible?: boolean): void;
  static size(): Number;
  getCheckerLockArg(): PubKeyHash;
  getHeight(): BlockHeight;
}

export function SerializeCheckerLastAcceptTaskHeights(value: Array<object>): ArrayBuffer;
export class CheckerLastAcceptTaskHeights {
  constructor(reader: CanCastToArrayBuffer, options?: CreateOptions);
  validate(compatible?: boolean): void;
  indexAt(i: number): CheckerLastAcceptTaskHeight;
  length(): number;
}

export function SerializeSidechainStateCell(value: object): ArrayBuffer;
export class SidechainStateCell {
  constructor(reader: CanCastToArrayBuffer, options?: CreateOptions);
  validate(compatible?: boolean): void;
  getVersion(): Uint8;
  getSubmitSidechainBlockHeight(): BlockHeight;
  getWaitingJobs(): Jobs;
  getRandomSeed(): RandomSeed;
  getRandomOffset(): Uint8;
  getRandomCommit(): CommittedCheckerInfos;
  getPunishCheckers(): PunishedCheckers;
  getRecentBlockHeaders(): BlockHeaders;
  getAncientBlockHeardMerkleRoot(): MerkleHash;
  getCheckerLastTaskSidechainHeights(): CheckerLastAcceptTaskHeights;
}

export function SerializeSidechainStateCellTypeArgs(value: object): ArrayBuffer;
export class SidechainStateCellTypeArgs {
  constructor(reader: CanCastToArrayBuffer, options?: CreateOptions);
  validate(compatible?: boolean): void;
  static size(): Number;
  getChainId(): ChainId;
}

export function SerializeUint8(value: CanCastToArrayBuffer): ArrayBuffer;
export class Uint8 {
  constructor(reader: CanCastToArrayBuffer, options?: CreateOptions);
  validate(compatible?: boolean): void;
  indexAt(i: number): number;
  raw(): ArrayBuffer;
  static size(): Number;
}

export function SerializeUint16(value: CanCastToArrayBuffer): ArrayBuffer;
export class Uint16 {
  constructor(reader: CanCastToArrayBuffer, options?: CreateOptions);
  validate(compatible?: boolean): void;
  indexAt(i: number): number;
  raw(): ArrayBuffer;
  toBigEndianUint16(): number;
  toLittleEndianUint16(): number;
  static size(): Number;
}

export function SerializeUint32(value: CanCastToArrayBuffer): ArrayBuffer;
export class Uint32 {
  constructor(reader: CanCastToArrayBuffer, options?: CreateOptions);
  validate(compatible?: boolean): void;
  indexAt(i: number): number;
  raw(): ArrayBuffer;
  toBigEndianUint32(): number;
  toLittleEndianUint32(): number;
  static size(): Number;
}

export function SerializeUint64(value: CanCastToArrayBuffer): ArrayBuffer;
export class Uint64 {
  constructor(reader: CanCastToArrayBuffer, options?: CreateOptions);
  validate(compatible?: boolean): void;
  indexAt(i: number): number;
  raw(): ArrayBuffer;
  toBigEndianBigUint64(): bigint;
  toLittleEndianBigUint64(): bigint;
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

export function SerializeBytes16(value: CanCastToArrayBuffer): ArrayBuffer;
export class Bytes16 {
  constructor(reader: CanCastToArrayBuffer, options?: CreateOptions);
  validate(compatible?: boolean): void;
  indexAt(i: number): number;
  raw(): ArrayBuffer;
  static size(): Number;
}

export function SerializeBytes32(value: CanCastToArrayBuffer): ArrayBuffer;
export class Bytes32 {
  constructor(reader: CanCastToArrayBuffer, options?: CreateOptions);
  validate(compatible?: boolean): void;
  indexAt(i: number): number;
  raw(): ArrayBuffer;
  static size(): Number;
}

export function SerializeBlockHeader(value: CanCastToArrayBuffer): ArrayBuffer;
export class BlockHeader {
  constructor(reader: CanCastToArrayBuffer, options?: CreateOptions);
  validate(compatible?: boolean): void;
  indexAt(i: number): number;
  raw(): ArrayBuffer;
  static size(): Number;
}

export function SerializeBlockHeight(value: CanCastToArrayBuffer): ArrayBuffer;
export class BlockHeight {
  constructor(reader: CanCastToArrayBuffer, options?: CreateOptions);
  validate(compatible?: boolean): void;
  indexAt(i: number): number;
  raw(): ArrayBuffer;
  static size(): Number;
}

export function SerializeCodeHash(value: CanCastToArrayBuffer): ArrayBuffer;
export class CodeHash {
  constructor(reader: CanCastToArrayBuffer, options?: CreateOptions);
  validate(compatible?: boolean): void;
  indexAt(i: number): number;
  raw(): ArrayBuffer;
  static size(): Number;
}

export function SerializeHashType(value: CanCastToArrayBuffer): ArrayBuffer;
export class HashType {
  constructor(reader: CanCastToArrayBuffer, options?: CreateOptions);
  validate(compatible?: boolean): void;
  indexAt(i: number): number;
  raw(): ArrayBuffer;
  static size(): Number;
}

export function SerializeMerkleHash(value: CanCastToArrayBuffer): ArrayBuffer;
export class MerkleHash {
  constructor(reader: CanCastToArrayBuffer, options?: CreateOptions);
  validate(compatible?: boolean): void;
  indexAt(i: number): number;
  raw(): ArrayBuffer;
  static size(): Number;
}

export function SerializePubKeyHash(value: CanCastToArrayBuffer): ArrayBuffer;
export class PubKeyHash {
  constructor(reader: CanCastToArrayBuffer, options?: CreateOptions);
  validate(compatible?: boolean): void;
  indexAt(i: number): number;
  raw(): ArrayBuffer;
  static size(): Number;
}

export function SerializeScriptHash(value: CanCastToArrayBuffer): ArrayBuffer;
export class ScriptHash {
  constructor(reader: CanCastToArrayBuffer, options?: CreateOptions);
  validate(compatible?: boolean): void;
  indexAt(i: number): number;
  raw(): ArrayBuffer;
  static size(): Number;
}

export function SerializePubKeyHashList(value: Array<CanCastToArrayBuffer>): ArrayBuffer;
export class PubKeyHashList {
  constructor(reader: CanCastToArrayBuffer, options?: CreateOptions);
  validate(compatible?: boolean): void;
  indexAt(i: number): PubKeyHash;
  length(): number;
}

export function SerializeBlockSlice(value: object): ArrayBuffer;
export class BlockSlice {
  constructor(reader: CanCastToArrayBuffer, options?: CreateOptions);
  validate(compatible?: boolean): void;
  static size(): Number;
  getFrom(): BlockHeight;
  getTo(): BlockHeight;
}

export function SerializeChainId(value: CanCastToArrayBuffer): ArrayBuffer;
export class ChainId {
  constructor(reader: CanCastToArrayBuffer, options?: CreateOptions);
  validate(compatible?: boolean): void;
  indexAt(i: number): number;
  raw(): ArrayBuffer;
  toBigEndianUint32(): number;
  toLittleEndianUint32(): number;
  static size(): Number;
}

export function SerializeChainIdList(value: Array<CanCastToArrayBuffer>): ArrayBuffer;
export class ChainIdList {
  constructor(reader: CanCastToArrayBuffer, options?: CreateOptions);
  validate(compatible?: boolean): void;
  indexAt(i: number): ChainId;
  length(): number;
}

export function SerializeRandomSeed(value: CanCastToArrayBuffer): ArrayBuffer;
export class RandomSeed {
  constructor(reader: CanCastToArrayBuffer, options?: CreateOptions);
  validate(compatible?: boolean): void;
  indexAt(i: number): number;
  raw(): ArrayBuffer;
  static size(): Number;
}

export function SerializeCommittedHash(value: CanCastToArrayBuffer): ArrayBuffer;
export class CommittedHash {
  constructor(reader: CanCastToArrayBuffer, options?: CreateOptions);
  validate(compatible?: boolean): void;
  indexAt(i: number): number;
  raw(): ArrayBuffer;
  static size(): Number;
}

export function SerializeMolString(value: CanCastToArrayBuffer): ArrayBuffer;
export class MolString {
  constructor(reader: CanCastToArrayBuffer, options?: CreateOptions);
  validate(compatible?: boolean): void;
  indexAt(i: number): number;
  raw(): ArrayBuffer;
  length(): number;
}

export function SerializeUint8Opt(value: CanCastToArrayBuffer | null): ArrayBuffer;
export class Uint8Opt {
  constructor(reader: CanCastToArrayBuffer, options?: CreateOptions);
  validate(compatible?: boolean): void;
  value(): Uint8;
  hasValue(): boolean;
}

export function SerializeUint16Opt(value: CanCastToArrayBuffer | null): ArrayBuffer;
export class Uint16Opt {
  constructor(reader: CanCastToArrayBuffer, options?: CreateOptions);
  validate(compatible?: boolean): void;
  value(): Uint16;
  hasValue(): boolean;
}

export function SerializeUint32Opt(value: CanCastToArrayBuffer | null): ArrayBuffer;
export class Uint32Opt {
  constructor(reader: CanCastToArrayBuffer, options?: CreateOptions);
  validate(compatible?: boolean): void;
  value(): Uint32;
  hasValue(): boolean;
}

export function SerializeUint64Opt(value: CanCastToArrayBuffer | null): ArrayBuffer;
export class Uint64Opt {
  constructor(reader: CanCastToArrayBuffer, options?: CreateOptions);
  validate(compatible?: boolean): void;
  value(): Uint64;
  hasValue(): boolean;
}

export function SerializeUint128Opt(value: CanCastToArrayBuffer | null): ArrayBuffer;
export class Uint128Opt {
  constructor(reader: CanCastToArrayBuffer, options?: CreateOptions);
  validate(compatible?: boolean): void;
  value(): Uint128;
  hasValue(): boolean;
}

export function SerializeCommittedHashOpt(value: CanCastToArrayBuffer | null): ArrayBuffer;
export class CommittedHashOpt {
  constructor(reader: CanCastToArrayBuffer, options?: CreateOptions);
  validate(compatible?: boolean): void;
  value(): CommittedHash;
  hasValue(): boolean;
}

