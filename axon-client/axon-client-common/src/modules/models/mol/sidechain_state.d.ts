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
  static size(): number;
  getCheckerLockArg(): PubKeyHash;
  getCommittedHash(): CommittedHash;
}

export function SerializePunishedChecker(value: object): ArrayBuffer;
export class PunishedChecker {
  constructor(reader: CanCastToArrayBuffer, options?: CreateOptions);
  validate(compatible?: boolean): void;
  static size(): number;
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
  static size(): number;
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
  getConfirmedJobs(): Jobs;
  getRandomSeed(): RandomSeed;
  getRandomOffset(): Uint8;
  getRandomCommit(): CommittedCheckerInfo;
  getPunishCheckers(): PunishedCheckers;
  getRecentBlockHeaders(): BlockHeaders;
  getAncientBlockHeardMerkleRoot(): MerkleHash;
  getCheckerLastTaskSidechainHeights(): CheckerLastAcceptTaskHeights;
}

export function SerializeSidechainStateCellLockArgs(value: object): ArrayBuffer;
export class SidechainStateCellLockArgs {
  constructor(reader: CanCastToArrayBuffer, options?: CreateOptions);
  validate(compatible?: boolean): void;
  static size(): number;
  getChainId(): ChainId;
}

export function SerializeUint8(value: CanCastToArrayBuffer): ArrayBuffer;
export class Uint8 {
  constructor(reader: CanCastToArrayBuffer, options?: CreateOptions);
  validate(compatible?: boolean): void;
  indexAt(i: number): number;
  raw(): ArrayBuffer;
  static size(): number;
}

export function SerializeUint16(value: CanCastToArrayBuffer): ArrayBuffer;
export class Uint16 {
  constructor(reader: CanCastToArrayBuffer, options?: CreateOptions);
  validate(compatible?: boolean): void;
  indexAt(i: number): number;
  raw(): ArrayBuffer;
  toBigEndianUint16(): number;
  toLittleEndianUint16(): number;
  static size(): number;
}

export function SerializeUint32(value: CanCastToArrayBuffer): ArrayBuffer;
export class Uint32 {
  constructor(reader: CanCastToArrayBuffer, options?: CreateOptions);
  validate(compatible?: boolean): void;
  indexAt(i: number): number;
  raw(): ArrayBuffer;
  toBigEndianUint32(): number;
  toLittleEndianUint32(): number;
  static size(): number;
}

export function SerializeUint64(value: CanCastToArrayBuffer): ArrayBuffer;
export class Uint64 {
  constructor(reader: CanCastToArrayBuffer, options?: CreateOptions);
  validate(compatible?: boolean): void;
  indexAt(i: number): number;
  raw(): ArrayBuffer;
  toBigEndianBigUint64(): bigint;
  toLittleEndianBigUint64(): bigint;
  static size(): number;
}

export function SerializeUint128(value: CanCastToArrayBuffer): ArrayBuffer;
export class Uint128 {
  constructor(reader: CanCastToArrayBuffer, options?: CreateOptions);
  validate(compatible?: boolean): void;
  indexAt(i: number): number;
  raw(): ArrayBuffer;
  static size(): number;
}

export function SerializeBytes16(value: CanCastToArrayBuffer): ArrayBuffer;
export class Bytes16 {
  constructor(reader: CanCastToArrayBuffer, options?: CreateOptions);
  validate(compatible?: boolean): void;
  indexAt(i: number): number;
  raw(): ArrayBuffer;
  static size(): number;
}

export function SerializeBytes32(value: CanCastToArrayBuffer): ArrayBuffer;
export class Bytes32 {
  constructor(reader: CanCastToArrayBuffer, options?: CreateOptions);
  validate(compatible?: boolean): void;
  indexAt(i: number): number;
  raw(): ArrayBuffer;
  static size(): number;
}

export function SerializeBlockHeader(value: CanCastToArrayBuffer): ArrayBuffer;
export class BlockHeader {
  constructor(reader: CanCastToArrayBuffer, options?: CreateOptions);
  validate(compatible?: boolean): void;
  indexAt(i: number): number;
  raw(): ArrayBuffer;
  static size(): number;
}

export function SerializeBlockHeight(value: CanCastToArrayBuffer): ArrayBuffer;
export class BlockHeight {
  constructor(reader: CanCastToArrayBuffer, options?: CreateOptions);
  validate(compatible?: boolean): void;
  indexAt(i: number): number;
  raw(): ArrayBuffer;
  static size(): number;
}

export function SerializeCodeHash(value: CanCastToArrayBuffer): ArrayBuffer;
export class CodeHash {
  constructor(reader: CanCastToArrayBuffer, options?: CreateOptions);
  validate(compatible?: boolean): void;
  indexAt(i: number): number;
  raw(): ArrayBuffer;
  static size(): number;
}

export function SerializeHashType(value: CanCastToArrayBuffer): ArrayBuffer;
export class HashType {
  constructor(reader: CanCastToArrayBuffer, options?: CreateOptions);
  validate(compatible?: boolean): void;
  indexAt(i: number): number;
  raw(): ArrayBuffer;
  static size(): number;
}

export function SerializeMerkleHash(value: CanCastToArrayBuffer): ArrayBuffer;
export class MerkleHash {
  constructor(reader: CanCastToArrayBuffer, options?: CreateOptions);
  validate(compatible?: boolean): void;
  indexAt(i: number): number;
  raw(): ArrayBuffer;
  static size(): number;
}

export function SerializePubKeyHash(value: CanCastToArrayBuffer): ArrayBuffer;
export class PubKeyHash {
  constructor(reader: CanCastToArrayBuffer, options?: CreateOptions);
  validate(compatible?: boolean): void;
  indexAt(i: number): number;
  raw(): ArrayBuffer;
  static size(): number;
}

export function SerializeScriptHash(value: CanCastToArrayBuffer): ArrayBuffer;
export class ScriptHash {
  constructor(reader: CanCastToArrayBuffer, options?: CreateOptions);
  validate(compatible?: boolean): void;
  indexAt(i: number): number;
  raw(): ArrayBuffer;
  static size(): number;
}

export function SerializeBlockSlice(value: object): ArrayBuffer;
export class BlockSlice {
  constructor(reader: CanCastToArrayBuffer, options?: CreateOptions);
  validate(compatible?: boolean): void;
  static size(): number;
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
  static size(): number;
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
  static size(): number;
}

export function SerializeCommittedHash(value: CanCastToArrayBuffer): ArrayBuffer;
export class CommittedHash {
  constructor(reader: CanCastToArrayBuffer, options?: CreateOptions);
  validate(compatible?: boolean): void;
  indexAt(i: number): number;
  raw(): ArrayBuffer;
  static size(): number;
}

export function SerializeMolString(value: CanCastToArrayBuffer): ArrayBuffer;
export class MolString {
  constructor(reader: CanCastToArrayBuffer, options?: CreateOptions);
  validate(compatible?: boolean): void;
  indexAt(i: number): number;
  raw(): ArrayBuffer;
  length(): number;
}
