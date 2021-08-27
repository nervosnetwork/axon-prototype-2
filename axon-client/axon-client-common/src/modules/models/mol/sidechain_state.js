function dataLengthError(actual, required) {
    throw new Error(`Invalid data length! Required: ${required}, actual: ${actual}`);
}

function assertDataLength(actual, required) {
  if (actual !== required) {
    dataLengthError(actual, required);
  }
}

function assertArrayBuffer(reader) {
  if (reader instanceof Object && reader.toArrayBuffer instanceof Function) {
    reader = reader.toArrayBuffer();
  }
  if (!(reader instanceof ArrayBuffer)) {
    throw new Error("Provided value must be an ArrayBuffer or can be transformed into ArrayBuffer!");
  }
  return reader;
}

function verifyAndExtractOffsets(view, expectedFieldCount, compatible) {
  if (view.byteLength < 4) {
    dataLengthError(view.byteLength, ">4");
  }
  const requiredByteLength = view.getUint32(0, true);
  assertDataLength(view.byteLength, requiredByteLength);
  if (requiredByteLength === 4) {
    return [requiredByteLength];
  }
  if (requiredByteLength < 8) {
    dataLengthError(view.byteLength, ">8");
  }
  const firstOffset = view.getUint32(4, true);
  if (firstOffset % 4 !== 0 || firstOffset < 8) {
    throw new Error(`Invalid first offset: ${firstOffset}`);
  }
  const itemCount = firstOffset / 4 - 1;
  if (itemCount < expectedFieldCount) {
    throw new Error(`Item count not enough! Required: ${expectedFieldCount}, actual: ${itemCount}`);
  } else if ((!compatible) && itemCount > expectedFieldCount) {
    throw new Error(`Item count is more than required! Required: ${expectedFieldCount}, actual: ${itemCount}`);
  }
  if (requiredByteLength < firstOffset) {
    throw new Error(`First offset is larger than byte length: ${firstOffset}`);
  }
  const offsets = [];
  for (let i = 0; i < itemCount; i++) {
    const start = 4 + i * 4;
    offsets.push(view.getUint32(start, true));
  }
  offsets.push(requiredByteLength);
  for (let i = 0; i < offsets.length - 1; i++) {
    if (offsets[i] > offsets[i + 1]) {
      throw new Error(`Offset index ${i}: ${offsets[i]} is larger than offset index ${i + 1}: ${offsets[i + 1]}`);
    }
  }
  return offsets;
}

function serializeTable(buffers) {
  const itemCount = buffers.length;
  let totalSize = 4 * (itemCount + 1);
  const offsets = [];

  for (let i = 0; i < itemCount; i++) {
    offsets.push(totalSize);
    totalSize += buffers[i].byteLength;
  }

  const buffer = new ArrayBuffer(totalSize);
  const array = new Uint8Array(buffer);
  const view = new DataView(buffer);

  view.setUint32(0, totalSize, true);
  for (let i = 0; i < itemCount; i++) {
    view.setUint32(4 + i * 4, offsets[i], true);
    array.set(new Uint8Array(buffers[i]), offsets[i]);
  }
  return buffer;
}

export class Jobs {
  constructor(reader, { validate = true } = {}) {
    this.view = new DataView(assertArrayBuffer(reader));
    if (validate) {
      this.validate();
    }
  }

  validate(compatible = false) {
    if (this.view.byteLength < 4) {
      dataLengthError(this.view.byteLength, ">4");
    }
    const requiredByteLength = this.length() * BlockSlice.size() + 4;
    assertDataLength(this.view.byteLength, requiredByteLength);
    for (let i = 0; i < 0; i++) {
      const item = this.indexAt(i);
      item.validate(compatible);
    }
  }

  indexAt(i) {
    return new BlockSlice(this.view.buffer.slice(4 + i * BlockSlice.size(), 4 + (i + 1) * BlockSlice.size()), { validate: false });
  }

  length() {
    return this.view.getUint32(0, true);
  }
}

export function SerializeJobs(value) {
  const array = new Uint8Array(4 + BlockSlice.size() * value.length);
  (new DataView(array.buffer)).setUint32(0, value.length, true);
  for (let i = 0; i < value.length; i++) {
    const itemBuffer = SerializeBlockSlice(value[i]);
    array.set(new Uint8Array(itemBuffer), 4 + i * BlockSlice.size());
  }
  return array.buffer;
}

export class CommittedCheckerInfo {
  constructor(reader, { validate = true } = {}) {
    this.view = new DataView(assertArrayBuffer(reader));
    if (validate) {
      this.validate();
    }
  }

  getCheckerLockArg() {
    return new PubKeyHash(this.view.buffer.slice(0, 0 + PubKeyHash.size()), { validate: false });
  }

  getCommittedHash() {
    return new CommittedHash(this.view.buffer.slice(0 + PubKeyHash.size(), 0 + PubKeyHash.size() + CommittedHash.size()), { validate: false });
  }

  validate(compatible = false) {
    assertDataLength(this.view.byteLength, CommittedCheckerInfo.size());
    this.getCheckerLockArg().validate(compatible);
    this.getCommittedHash().validate(compatible);
  }
  static size() {
    return 0 + PubKeyHash.size() + CommittedHash.size();
  }
}

export function SerializeCommittedCheckerInfo(value) {
  const array = new Uint8Array(0 + PubKeyHash.size() + CommittedHash.size());
  const view = new DataView(array.buffer);
  array.set(new Uint8Array(SerializePubKeyHash(value.checker_lock_arg)), 0);
  array.set(new Uint8Array(SerializeCommittedHash(value.committed_hash)), 0 + PubKeyHash.size());
  return array.buffer;
}

export class CommittedCheckerInfos {
  constructor(reader, { validate = true } = {}) {
    this.view = new DataView(assertArrayBuffer(reader));
    if (validate) {
      this.validate();
    }
  }

  validate(compatible = false) {
    if (this.view.byteLength < 4) {
      dataLengthError(this.view.byteLength, ">4");
    }
    const requiredByteLength = this.length() * CommittedCheckerInfo.size() + 4;
    assertDataLength(this.view.byteLength, requiredByteLength);
    for (let i = 0; i < 0; i++) {
      const item = this.indexAt(i);
      item.validate(compatible);
    }
  }

  indexAt(i) {
    return new CommittedCheckerInfo(this.view.buffer.slice(4 + i * CommittedCheckerInfo.size(), 4 + (i + 1) * CommittedCheckerInfo.size()), { validate: false });
  }

  length() {
    return this.view.getUint32(0, true);
  }
}

export function SerializeCommittedCheckerInfos(value) {
  const array = new Uint8Array(4 + CommittedCheckerInfo.size() * value.length);
  (new DataView(array.buffer)).setUint32(0, value.length, true);
  for (let i = 0; i < value.length; i++) {
    const itemBuffer = SerializeCommittedCheckerInfo(value[i]);
    array.set(new Uint8Array(itemBuffer), 4 + i * CommittedCheckerInfo.size());
  }
  return array.buffer;
}

export class PunishedChecker {
  constructor(reader, { validate = true } = {}) {
    this.view = new DataView(assertArrayBuffer(reader));
    if (validate) {
      this.validate();
    }
  }

  getCheckerLockArg() {
    return new PubKeyHash(this.view.buffer.slice(0, 0 + PubKeyHash.size()), { validate: false });
  }

  getPunishPoints() {
    return new Uint32(this.view.buffer.slice(0 + PubKeyHash.size(), 0 + PubKeyHash.size() + Uint32.size()), { validate: false });
  }

  validate(compatible = false) {
    assertDataLength(this.view.byteLength, PunishedChecker.size());
    this.getCheckerLockArg().validate(compatible);
    this.getPunishPoints().validate(compatible);
  }
  static size() {
    return 0 + PubKeyHash.size() + Uint32.size();
  }
}

export function SerializePunishedChecker(value) {
  const array = new Uint8Array(0 + PubKeyHash.size() + Uint32.size());
  const view = new DataView(array.buffer);
  array.set(new Uint8Array(SerializePubKeyHash(value.checker_lock_arg)), 0);
  array.set(new Uint8Array(SerializeUint32(value.punish_points)), 0 + PubKeyHash.size());
  return array.buffer;
}

export class PunishedCheckers {
  constructor(reader, { validate = true } = {}) {
    this.view = new DataView(assertArrayBuffer(reader));
    if (validate) {
      this.validate();
    }
  }

  validate(compatible = false) {
    if (this.view.byteLength < 4) {
      dataLengthError(this.view.byteLength, ">4");
    }
    const requiredByteLength = this.length() * PunishedChecker.size() + 4;
    assertDataLength(this.view.byteLength, requiredByteLength);
    for (let i = 0; i < 0; i++) {
      const item = this.indexAt(i);
      item.validate(compatible);
    }
  }

  indexAt(i) {
    return new PunishedChecker(this.view.buffer.slice(4 + i * PunishedChecker.size(), 4 + (i + 1) * PunishedChecker.size()), { validate: false });
  }

  length() {
    return this.view.getUint32(0, true);
  }
}

export function SerializePunishedCheckers(value) {
  const array = new Uint8Array(4 + PunishedChecker.size() * value.length);
  (new DataView(array.buffer)).setUint32(0, value.length, true);
  for (let i = 0; i < value.length; i++) {
    const itemBuffer = SerializePunishedChecker(value[i]);
    array.set(new Uint8Array(itemBuffer), 4 + i * PunishedChecker.size());
  }
  return array.buffer;
}

export class BlockHeaders {
  constructor(reader, { validate = true } = {}) {
    this.view = new DataView(assertArrayBuffer(reader));
    if (validate) {
      this.validate();
    }
  }

  validate(compatible = false) {
    if (this.view.byteLength < 4) {
      dataLengthError(this.view.byteLength, ">4");
    }
    const requiredByteLength = this.length() * BlockHeader.size() + 4;
    assertDataLength(this.view.byteLength, requiredByteLength);
    for (let i = 0; i < 0; i++) {
      const item = this.indexAt(i);
      item.validate(compatible);
    }
  }

  indexAt(i) {
    return new BlockHeader(this.view.buffer.slice(4 + i * BlockHeader.size(), 4 + (i + 1) * BlockHeader.size()), { validate: false });
  }

  length() {
    return this.view.getUint32(0, true);
  }
}

export function SerializeBlockHeaders(value) {
  const array = new Uint8Array(4 + BlockHeader.size() * value.length);
  (new DataView(array.buffer)).setUint32(0, value.length, true);
  for (let i = 0; i < value.length; i++) {
    const itemBuffer = SerializeBlockHeader(value[i]);
    array.set(new Uint8Array(itemBuffer), 4 + i * BlockHeader.size());
  }
  return array.buffer;
}

export class CheckerLastAcceptTaskHeight {
  constructor(reader, { validate = true } = {}) {
    this.view = new DataView(assertArrayBuffer(reader));
    if (validate) {
      this.validate();
    }
  }

  getCheckerLockArg() {
    return new PubKeyHash(this.view.buffer.slice(0, 0 + PubKeyHash.size()), { validate: false });
  }

  getHeight() {
    return new BlockHeight(this.view.buffer.slice(0 + PubKeyHash.size(), 0 + PubKeyHash.size() + BlockHeight.size()), { validate: false });
  }

  validate(compatible = false) {
    assertDataLength(this.view.byteLength, CheckerLastAcceptTaskHeight.size());
    this.getCheckerLockArg().validate(compatible);
    this.getHeight().validate(compatible);
  }
  static size() {
    return 0 + PubKeyHash.size() + BlockHeight.size();
  }
}

export function SerializeCheckerLastAcceptTaskHeight(value) {
  const array = new Uint8Array(0 + PubKeyHash.size() + BlockHeight.size());
  const view = new DataView(array.buffer);
  array.set(new Uint8Array(SerializePubKeyHash(value.checker_lock_arg)), 0);
  array.set(new Uint8Array(SerializeBlockHeight(value.height)), 0 + PubKeyHash.size());
  return array.buffer;
}

export class CheckerLastAcceptTaskHeights {
  constructor(reader, { validate = true } = {}) {
    this.view = new DataView(assertArrayBuffer(reader));
    if (validate) {
      this.validate();
    }
  }

  validate(compatible = false) {
    if (this.view.byteLength < 4) {
      dataLengthError(this.view.byteLength, ">4");
    }
    const requiredByteLength = this.length() * CheckerLastAcceptTaskHeight.size() + 4;
    assertDataLength(this.view.byteLength, requiredByteLength);
    for (let i = 0; i < 0; i++) {
      const item = this.indexAt(i);
      item.validate(compatible);
    }
  }

  indexAt(i) {
    return new CheckerLastAcceptTaskHeight(this.view.buffer.slice(4 + i * CheckerLastAcceptTaskHeight.size(), 4 + (i + 1) * CheckerLastAcceptTaskHeight.size()), { validate: false });
  }

  length() {
    return this.view.getUint32(0, true);
  }
}

export function SerializeCheckerLastAcceptTaskHeights(value) {
  const array = new Uint8Array(4 + CheckerLastAcceptTaskHeight.size() * value.length);
  (new DataView(array.buffer)).setUint32(0, value.length, true);
  for (let i = 0; i < value.length; i++) {
    const itemBuffer = SerializeCheckerLastAcceptTaskHeight(value[i]);
    array.set(new Uint8Array(itemBuffer), 4 + i * CheckerLastAcceptTaskHeight.size());
  }
  return array.buffer;
}

export class SidechainStateCell {
  constructor(reader, { validate = true } = {}) {
    this.view = new DataView(assertArrayBuffer(reader));
    if (validate) {
      this.validate();
    }
  }

  validate(compatible = false) {
    const offsets = verifyAndExtractOffsets(this.view, 0, true);
    new Uint8(this.view.buffer.slice(offsets[0], offsets[1]), { validate: false }).validate();
    new BlockHeight(this.view.buffer.slice(offsets[1], offsets[2]), { validate: false }).validate();
    new Jobs(this.view.buffer.slice(offsets[2], offsets[3]), { validate: false }).validate();
    new RandomSeed(this.view.buffer.slice(offsets[3], offsets[4]), { validate: false }).validate();
    new Uint8(this.view.buffer.slice(offsets[4], offsets[5]), { validate: false }).validate();
    new CommittedCheckerInfos(this.view.buffer.slice(offsets[5], offsets[6]), { validate: false }).validate();
    new PunishedCheckers(this.view.buffer.slice(offsets[6], offsets[7]), { validate: false }).validate();
    new BlockHeaders(this.view.buffer.slice(offsets[7], offsets[8]), { validate: false }).validate();
    new MerkleHash(this.view.buffer.slice(offsets[8], offsets[9]), { validate: false }).validate();
    new CheckerLastAcceptTaskHeights(this.view.buffer.slice(offsets[9], offsets[10]), { validate: false }).validate();
  }

  getVersion() {
    const start = 4;
    const offset = this.view.getUint32(start, true);
    const offset_end = this.view.getUint32(start + 4, true);
    return new Uint8(this.view.buffer.slice(offset, offset_end), { validate: false });
  }

  getSubmitSidechainBlockHeight() {
    const start = 8;
    const offset = this.view.getUint32(start, true);
    const offset_end = this.view.getUint32(start + 4, true);
    return new BlockHeight(this.view.buffer.slice(offset, offset_end), { validate: false });
  }

  getWaitingJobs() {
    const start = 12;
    const offset = this.view.getUint32(start, true);
    const offset_end = this.view.getUint32(start + 4, true);
    return new Jobs(this.view.buffer.slice(offset, offset_end), { validate: false });
  }

  getRandomSeed() {
    const start = 16;
    const offset = this.view.getUint32(start, true);
    const offset_end = this.view.getUint32(start + 4, true);
    return new RandomSeed(this.view.buffer.slice(offset, offset_end), { validate: false });
  }

  getRandomOffset() {
    const start = 20;
    const offset = this.view.getUint32(start, true);
    const offset_end = this.view.getUint32(start + 4, true);
    return new Uint8(this.view.buffer.slice(offset, offset_end), { validate: false });
  }

  getRandomCommit() {
    const start = 24;
    const offset = this.view.getUint32(start, true);
    const offset_end = this.view.getUint32(start + 4, true);
    return new CommittedCheckerInfos(this.view.buffer.slice(offset, offset_end), { validate: false });
  }

  getPunishCheckers() {
    const start = 28;
    const offset = this.view.getUint32(start, true);
    const offset_end = this.view.getUint32(start + 4, true);
    return new PunishedCheckers(this.view.buffer.slice(offset, offset_end), { validate: false });
  }

  getRecentBlockHeaders() {
    const start = 32;
    const offset = this.view.getUint32(start, true);
    const offset_end = this.view.getUint32(start + 4, true);
    return new BlockHeaders(this.view.buffer.slice(offset, offset_end), { validate: false });
  }

  getAncientBlockHeardMerkleRoot() {
    const start = 36;
    const offset = this.view.getUint32(start, true);
    const offset_end = this.view.getUint32(start + 4, true);
    return new MerkleHash(this.view.buffer.slice(offset, offset_end), { validate: false });
  }

  getCheckerLastTaskSidechainHeights() {
    const start = 40;
    const offset = this.view.getUint32(start, true);
    const offset_end = this.view.byteLength;
    return new CheckerLastAcceptTaskHeights(this.view.buffer.slice(offset, offset_end), { validate: false });
  }
}

export function SerializeSidechainStateCell(value) {
  const buffers = [];
  buffers.push(SerializeUint8(value.version));
  buffers.push(SerializeBlockHeight(value.submit_sidechain_block_height));
  buffers.push(SerializeJobs(value.waiting_jobs));
  buffers.push(SerializeRandomSeed(value.random_seed));
  buffers.push(SerializeUint8(value.random_offset));
  buffers.push(SerializeCommittedCheckerInfos(value.random_commit));
  buffers.push(SerializePunishedCheckers(value.punish_checkers));
  buffers.push(SerializeBlockHeaders(value.recent_block_headers));
  buffers.push(SerializeMerkleHash(value.ancient_block_heard_merkle_root));
  buffers.push(SerializeCheckerLastAcceptTaskHeights(value.checker_last_task_sidechain_heights));
  return serializeTable(buffers);
}

export class SidechainStateCellTypeArgs {
  constructor(reader, { validate = true } = {}) {
    this.view = new DataView(assertArrayBuffer(reader));
    if (validate) {
      this.validate();
    }
  }

  getChainId() {
    return new ChainId(this.view.buffer.slice(0, 0 + ChainId.size()), { validate: false });
  }

  validate(compatible = false) {
    assertDataLength(this.view.byteLength, SidechainStateCellTypeArgs.size());
    this.getChainId().validate(compatible);
  }
  static size() {
    return 0 + ChainId.size();
  }
}

export function SerializeSidechainStateCellTypeArgs(value) {
  const array = new Uint8Array(0 + ChainId.size());
  const view = new DataView(array.buffer);
  array.set(new Uint8Array(SerializeChainId(value.chain_id)), 0);
  return array.buffer;
}

export class Uint8 {
  constructor(reader, { validate = true } = {}) {
    this.view = new DataView(assertArrayBuffer(reader));
    if (validate) {
      this.validate();
    }
  }

  validate(compatible = false) {
    assertDataLength(this.view.byteLength, 1);
  }

  indexAt(i) {
    return this.view.getUint8(i);
  }

  raw() {
    return this.view.buffer;
  }

  static size() {
    return 1;
  }
}

export function SerializeUint8(value) {
  const buffer = assertArrayBuffer(value);
  assertDataLength(buffer.byteLength, 1);
  return buffer;
}

export class Uint16 {
  constructor(reader, { validate = true } = {}) {
    this.view = new DataView(assertArrayBuffer(reader));
    if (validate) {
      this.validate();
    }
  }

  validate(compatible = false) {
    assertDataLength(this.view.byteLength, 2);
  }

  indexAt(i) {
    return this.view.getUint8(i);
  }

  raw() {
    return this.view.buffer;
  }

  toBigEndianUint16() {
    return this.view.getUint16(0, false);
  }

  toLittleEndianUint16() {
    return this.view.getUint16(0, true);
  }

  static size() {
    return 2;
  }
}

export function SerializeUint16(value) {
  const buffer = assertArrayBuffer(value);
  assertDataLength(buffer.byteLength, 2);
  return buffer;
}

export class Uint32 {
  constructor(reader, { validate = true } = {}) {
    this.view = new DataView(assertArrayBuffer(reader));
    if (validate) {
      this.validate();
    }
  }

  validate(compatible = false) {
    assertDataLength(this.view.byteLength, 4);
  }

  indexAt(i) {
    return this.view.getUint8(i);
  }

  raw() {
    return this.view.buffer;
  }

  toBigEndianUint32() {
    return this.view.getUint32(0, false);
  }

  toLittleEndianUint32() {
    return this.view.getUint32(0, true);
  }

  static size() {
    return 4;
  }
}

export function SerializeUint32(value) {
  const buffer = assertArrayBuffer(value);
  assertDataLength(buffer.byteLength, 4);
  return buffer;
}

export class Uint64 {
  constructor(reader, { validate = true } = {}) {
    this.view = new DataView(assertArrayBuffer(reader));
    if (validate) {
      this.validate();
    }
  }

  validate(compatible = false) {
    assertDataLength(this.view.byteLength, 8);
  }

  indexAt(i) {
    return this.view.getUint8(i);
  }

  raw() {
    return this.view.buffer;
  }

  toBigEndianBigUint64() {
    return this.view.getBigUint64(0, false);
  }

  toLittleEndianBigUint64() {
    return this.view.getBigUint64(0, true);
  }

  static size() {
    return 8;
  }
}

export function SerializeUint64(value) {
  const buffer = assertArrayBuffer(value);
  assertDataLength(buffer.byteLength, 8);
  return buffer;
}

export class Uint128 {
  constructor(reader, { validate = true } = {}) {
    this.view = new DataView(assertArrayBuffer(reader));
    if (validate) {
      this.validate();
    }
  }

  validate(compatible = false) {
    assertDataLength(this.view.byteLength, 16);
  }

  indexAt(i) {
    return this.view.getUint8(i);
  }

  raw() {
    return this.view.buffer;
  }

  static size() {
    return 16;
  }
}

export function SerializeUint128(value) {
  const buffer = assertArrayBuffer(value);
  assertDataLength(buffer.byteLength, 16);
  return buffer;
}

export class Bytes16 {
  constructor(reader, { validate = true } = {}) {
    this.view = new DataView(assertArrayBuffer(reader));
    if (validate) {
      this.validate();
    }
  }

  validate(compatible = false) {
    assertDataLength(this.view.byteLength, 16);
  }

  indexAt(i) {
    return this.view.getUint8(i);
  }

  raw() {
    return this.view.buffer;
  }

  static size() {
    return 16;
  }
}

export function SerializeBytes16(value) {
  const buffer = assertArrayBuffer(value);
  assertDataLength(buffer.byteLength, 16);
  return buffer;
}

export class Bytes32 {
  constructor(reader, { validate = true } = {}) {
    this.view = new DataView(assertArrayBuffer(reader));
    if (validate) {
      this.validate();
    }
  }

  validate(compatible = false) {
    assertDataLength(this.view.byteLength, 32);
  }

  indexAt(i) {
    return this.view.getUint8(i);
  }

  raw() {
    return this.view.buffer;
  }

  static size() {
    return 32;
  }
}

export function SerializeBytes32(value) {
  const buffer = assertArrayBuffer(value);
  assertDataLength(buffer.byteLength, 32);
  return buffer;
}

export class BlockHeader {
  constructor(reader, { validate = true } = {}) {
    this.view = new DataView(assertArrayBuffer(reader));
    if (validate) {
      this.validate();
    }
  }

  validate(compatible = false) {
    assertDataLength(this.view.byteLength, 32);
  }

  indexAt(i) {
    return this.view.getUint8(i);
  }

  raw() {
    return this.view.buffer;
  }

  static size() {
    return 32;
  }
}

export function SerializeBlockHeader(value) {
  const buffer = assertArrayBuffer(value);
  assertDataLength(buffer.byteLength, 32);
  return buffer;
}

export class BlockHeight {
  constructor(reader, { validate = true } = {}) {
    this.view = new DataView(assertArrayBuffer(reader));
    if (validate) {
      this.validate();
    }
  }

  validate(compatible = false) {
    assertDataLength(this.view.byteLength, 16);
  }

  indexAt(i) {
    return this.view.getUint8(i);
  }

  raw() {
    return this.view.buffer;
  }

  static size() {
    return 16;
  }
}

export function SerializeBlockHeight(value) {
  const buffer = assertArrayBuffer(value);
  assertDataLength(buffer.byteLength, 16);
  return buffer;
}

export class CodeHash {
  constructor(reader, { validate = true } = {}) {
    this.view = new DataView(assertArrayBuffer(reader));
    if (validate) {
      this.validate();
    }
  }

  validate(compatible = false) {
    assertDataLength(this.view.byteLength, 32);
  }

  indexAt(i) {
    return this.view.getUint8(i);
  }

  raw() {
    return this.view.buffer;
  }

  static size() {
    return 32;
  }
}

export function SerializeCodeHash(value) {
  const buffer = assertArrayBuffer(value);
  assertDataLength(buffer.byteLength, 32);
  return buffer;
}

export class HashType {
  constructor(reader, { validate = true } = {}) {
    this.view = new DataView(assertArrayBuffer(reader));
    if (validate) {
      this.validate();
    }
  }

  validate(compatible = false) {
    assertDataLength(this.view.byteLength, 1);
  }

  indexAt(i) {
    return this.view.getUint8(i);
  }

  raw() {
    return this.view.buffer;
  }

  static size() {
    return 1;
  }
}

export function SerializeHashType(value) {
  const buffer = assertArrayBuffer(value);
  assertDataLength(buffer.byteLength, 1);
  return buffer;
}

export class MerkleHash {
  constructor(reader, { validate = true } = {}) {
    this.view = new DataView(assertArrayBuffer(reader));
    if (validate) {
      this.validate();
    }
  }

  validate(compatible = false) {
    assertDataLength(this.view.byteLength, 32);
  }

  indexAt(i) {
    return this.view.getUint8(i);
  }

  raw() {
    return this.view.buffer;
  }

  static size() {
    return 32;
  }
}

export function SerializeMerkleHash(value) {
  const buffer = assertArrayBuffer(value);
  assertDataLength(buffer.byteLength, 32);
  return buffer;
}

export class PubKeyHash {
  constructor(reader, { validate = true } = {}) {
    this.view = new DataView(assertArrayBuffer(reader));
    if (validate) {
      this.validate();
    }
  }

  validate(compatible = false) {
    assertDataLength(this.view.byteLength, 20);
  }

  indexAt(i) {
    return this.view.getUint8(i);
  }

  raw() {
    return this.view.buffer;
  }

  static size() {
    return 20;
  }
}

export function SerializePubKeyHash(value) {
  const buffer = assertArrayBuffer(value);
  assertDataLength(buffer.byteLength, 20);
  return buffer;
}

export class ScriptHash {
  constructor(reader, { validate = true } = {}) {
    this.view = new DataView(assertArrayBuffer(reader));
    if (validate) {
      this.validate();
    }
  }

  validate(compatible = false) {
    assertDataLength(this.view.byteLength, 32);
  }

  indexAt(i) {
    return this.view.getUint8(i);
  }

  raw() {
    return this.view.buffer;
  }

  static size() {
    return 32;
  }
}

export function SerializeScriptHash(value) {
  const buffer = assertArrayBuffer(value);
  assertDataLength(buffer.byteLength, 32);
  return buffer;
}

export class PubKeyHashList {
  constructor(reader, { validate = true } = {}) {
    this.view = new DataView(assertArrayBuffer(reader));
    if (validate) {
      this.validate();
    }
  }

  validate(compatible = false) {
    if (this.view.byteLength < 4) {
      dataLengthError(this.view.byteLength, ">4");
    }
    const requiredByteLength = this.length() * PubKeyHash.size() + 4;
    assertDataLength(this.view.byteLength, requiredByteLength);
    for (let i = 0; i < 0; i++) {
      const item = this.indexAt(i);
      item.validate(compatible);
    }
  }

  indexAt(i) {
    return new PubKeyHash(this.view.buffer.slice(4 + i * PubKeyHash.size(), 4 + (i + 1) * PubKeyHash.size()), { validate: false });
  }

  length() {
    return this.view.getUint32(0, true);
  }
}

export function SerializePubKeyHashList(value) {
  const array = new Uint8Array(4 + PubKeyHash.size() * value.length);
  (new DataView(array.buffer)).setUint32(0, value.length, true);
  for (let i = 0; i < value.length; i++) {
    const itemBuffer = SerializePubKeyHash(value[i]);
    array.set(new Uint8Array(itemBuffer), 4 + i * PubKeyHash.size());
  }
  return array.buffer;
}

export class BlockSlice {
  constructor(reader, { validate = true } = {}) {
    this.view = new DataView(assertArrayBuffer(reader));
    if (validate) {
      this.validate();
    }
  }

  getFrom() {
    return new BlockHeight(this.view.buffer.slice(0, 0 + BlockHeight.size()), { validate: false });
  }

  getTo() {
    return new BlockHeight(this.view.buffer.slice(0 + BlockHeight.size(), 0 + BlockHeight.size() + BlockHeight.size()), { validate: false });
  }

  validate(compatible = false) {
    assertDataLength(this.view.byteLength, BlockSlice.size());
    this.getFrom().validate(compatible);
    this.getTo().validate(compatible);
  }
  static size() {
    return 0 + BlockHeight.size() + BlockHeight.size();
  }
}

export function SerializeBlockSlice(value) {
  const array = new Uint8Array(0 + BlockHeight.size() + BlockHeight.size());
  const view = new DataView(array.buffer);
  array.set(new Uint8Array(SerializeBlockHeight(value.from)), 0);
  array.set(new Uint8Array(SerializeBlockHeight(value.to)), 0 + BlockHeight.size());
  return array.buffer;
}

export class ChainId {
  constructor(reader, { validate = true } = {}) {
    this.view = new DataView(assertArrayBuffer(reader));
    if (validate) {
      this.validate();
    }
  }

  validate(compatible = false) {
    assertDataLength(this.view.byteLength, 4);
  }

  indexAt(i) {
    return this.view.getUint8(i);
  }

  raw() {
    return this.view.buffer;
  }

  toBigEndianUint32() {
    return this.view.getUint32(0, false);
  }

  toLittleEndianUint32() {
    return this.view.getUint32(0, true);
  }

  static size() {
    return 4;
  }
}

export function SerializeChainId(value) {
  const buffer = assertArrayBuffer(value);
  assertDataLength(buffer.byteLength, 4);
  return buffer;
}

export class ChainIdList {
  constructor(reader, { validate = true } = {}) {
    this.view = new DataView(assertArrayBuffer(reader));
    if (validate) {
      this.validate();
    }
  }

  validate(compatible = false) {
    if (this.view.byteLength < 4) {
      dataLengthError(this.view.byteLength, ">4");
    }
    const requiredByteLength = this.length() * ChainId.size() + 4;
    assertDataLength(this.view.byteLength, requiredByteLength);
    for (let i = 0; i < 0; i++) {
      const item = this.indexAt(i);
      item.validate(compatible);
    }
  }

  indexAt(i) {
    return new ChainId(this.view.buffer.slice(4 + i * ChainId.size(), 4 + (i + 1) * ChainId.size()), { validate: false });
  }

  length() {
    return this.view.getUint32(0, true);
  }
}

export function SerializeChainIdList(value) {
  const array = new Uint8Array(4 + ChainId.size() * value.length);
  (new DataView(array.buffer)).setUint32(0, value.length, true);
  for (let i = 0; i < value.length; i++) {
    const itemBuffer = SerializeChainId(value[i]);
    array.set(new Uint8Array(itemBuffer), 4 + i * ChainId.size());
  }
  return array.buffer;
}

export class RandomSeed {
  constructor(reader, { validate = true } = {}) {
    this.view = new DataView(assertArrayBuffer(reader));
    if (validate) {
      this.validate();
    }
  }

  validate(compatible = false) {
    assertDataLength(this.view.byteLength, 32);
  }

  indexAt(i) {
    return this.view.getUint8(i);
  }

  raw() {
    return this.view.buffer;
  }

  static size() {
    return 32;
  }
}

export function SerializeRandomSeed(value) {
  const buffer = assertArrayBuffer(value);
  assertDataLength(buffer.byteLength, 32);
  return buffer;
}

export class CommittedHash {
  constructor(reader, { validate = true } = {}) {
    this.view = new DataView(assertArrayBuffer(reader));
    if (validate) {
      this.validate();
    }
  }

  validate(compatible = false) {
    assertDataLength(this.view.byteLength, 32);
  }

  indexAt(i) {
    return this.view.getUint8(i);
  }

  raw() {
    return this.view.buffer;
  }

  static size() {
    return 32;
  }
}

export function SerializeCommittedHash(value) {
  const buffer = assertArrayBuffer(value);
  assertDataLength(buffer.byteLength, 32);
  return buffer;
}

export class MolString {
  constructor(reader, { validate = true } = {}) {
    this.view = new DataView(assertArrayBuffer(reader));
    if (validate) {
      this.validate();
    }
  }

  validate(compatible = false) {
    if (this.view.byteLength < 4) {
      dataLengthError(this.view.byteLength, ">4")
    }
    const requiredByteLength = this.length() + 4;
    assertDataLength(this.view.byteLength, requiredByteLength);
  }

  raw() {
    return this.view.buffer.slice(4);
  }

  indexAt(i) {
    return this.view.getUint8(4 + i);
  }

  length() {
    return this.view.getUint32(0, true);
  }
}

export function SerializeMolString(value) {
  const item = assertArrayBuffer(value);
  const array = new Uint8Array(4 + item.byteLength);
  (new DataView(array.buffer)).setUint32(0, item.byteLength, true);
  array.set(new Uint8Array(item), 4);
  return array.buffer;
}

export class Uint8Opt {
  constructor(reader, { validate = true } = {}) {
    this.view = new DataView(assertArrayBuffer(reader));
    if (validate) {
      this.validate();
    }
  }

  validate(compatible = false) {
    if (this.hasValue()) {
      this.value().validate(compatible);
    }
  }

  value() {
    return new Uint8(this.view.buffer, { validate: false });
  }

  hasValue() {
    return this.view.byteLength > 0;
  }
}

export function SerializeUint8Opt(value) {
  if (value) {
    return SerializeUint8(value);
  } else {
    return new ArrayBuffer(0);
  }
}

export class Uint16Opt {
  constructor(reader, { validate = true } = {}) {
    this.view = new DataView(assertArrayBuffer(reader));
    if (validate) {
      this.validate();
    }
  }

  validate(compatible = false) {
    if (this.hasValue()) {
      this.value().validate(compatible);
    }
  }

  value() {
    return new Uint16(this.view.buffer, { validate: false });
  }

  hasValue() {
    return this.view.byteLength > 0;
  }
}

export function SerializeUint16Opt(value) {
  if (value) {
    return SerializeUint16(value);
  } else {
    return new ArrayBuffer(0);
  }
}

export class Uint32Opt {
  constructor(reader, { validate = true } = {}) {
    this.view = new DataView(assertArrayBuffer(reader));
    if (validate) {
      this.validate();
    }
  }

  validate(compatible = false) {
    if (this.hasValue()) {
      this.value().validate(compatible);
    }
  }

  value() {
    return new Uint32(this.view.buffer, { validate: false });
  }

  hasValue() {
    return this.view.byteLength > 0;
  }
}

export function SerializeUint32Opt(value) {
  if (value) {
    return SerializeUint32(value);
  } else {
    return new ArrayBuffer(0);
  }
}

export class Uint64Opt {
  constructor(reader, { validate = true } = {}) {
    this.view = new DataView(assertArrayBuffer(reader));
    if (validate) {
      this.validate();
    }
  }

  validate(compatible = false) {
    if (this.hasValue()) {
      this.value().validate(compatible);
    }
  }

  value() {
    return new Uint64(this.view.buffer, { validate: false });
  }

  hasValue() {
    return this.view.byteLength > 0;
  }
}

export function SerializeUint64Opt(value) {
  if (value) {
    return SerializeUint64(value);
  } else {
    return new ArrayBuffer(0);
  }
}

export class Uint128Opt {
  constructor(reader, { validate = true } = {}) {
    this.view = new DataView(assertArrayBuffer(reader));
    if (validate) {
      this.validate();
    }
  }

  validate(compatible = false) {
    if (this.hasValue()) {
      this.value().validate(compatible);
    }
  }

  value() {
    return new Uint128(this.view.buffer, { validate: false });
  }

  hasValue() {
    return this.view.byteLength > 0;
  }
}

export function SerializeUint128Opt(value) {
  if (value) {
    return SerializeUint128(value);
  } else {
    return new ArrayBuffer(0);
  }
}

export class CommittedHashOpt {
  constructor(reader, { validate = true } = {}) {
    this.view = new DataView(assertArrayBuffer(reader));
    if (validate) {
      this.validate();
    }
  }

  validate(compatible = false) {
    if (this.hasValue()) {
      this.value().validate(compatible);
    }
  }

  value() {
    return new CommittedHash(this.view.buffer, { validate: false });
  }

  hasValue() {
    return this.view.byteLength > 0;
  }
}

export function SerializeCommittedHashOpt(value) {
  if (value) {
    return SerializeCommittedHash(value);
  } else {
    return new ArrayBuffer(0);
  }
}

