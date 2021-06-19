function dataLengthError (actual, required) {
  throw new Error(`Invalid data length! Required: ${required}, actual: ${actual}`);
}

function assertDataLength (actual, required) {
  if (actual !== required) {
    dataLengthError(actual, required);
  }
}

function assertArrayBuffer (reader) {
  if (reader instanceof Object && reader.toArrayBuffer instanceof Function) {
    reader = reader.toArrayBuffer();
  }
  if (!(reader instanceof ArrayBuffer)) {
    throw new Error('Provided value must be an ArrayBuffer or can be transformed into ArrayBuffer!');
  }
  return reader;
}

function verifyAndExtractOffsets (view, expectedFieldCount, compatible) {
  if (view.byteLength < 4) {
    dataLengthError(view.byteLength, '>4');
  }
  const requiredByteLength = view.getUint32(0, true);
  assertDataLength(view.byteLength, requiredByteLength);
  if (requiredByteLength === 4) {
    return [requiredByteLength];
  }
  if (requiredByteLength < 8) {
    dataLengthError(view.byteLength, '>8');
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

function serializeTable (buffers) {
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

export class MintTokenWitness {
  constructor (reader, {validate = true} = {}) {
    this.view = new DataView(assertArrayBuffer(reader));
    if (validate) {
      this.validate();
    }
  }

  validate (compatible = false) {
    const offsets = verifyAndExtractOffsets(this.view, 0, true);
    if (offsets[1] - offsets[0] !== 1) {
      throw new Error(`Invalid offset for mode: ${offsets[0]} - ${offsets[1]}`);
    }
    new Bytes(this.view.buffer.slice(offsets[1], offsets[2]), {validate: false}).validate();
    new Bytes(this.view.buffer.slice(offsets[2], offsets[3]), {validate: false}).validate();
    new Bytes(this.view.buffer.slice(offsets[3], offsets[4]), {validate: false}).validate();
  }

  getMode () {
    const start = 4;
    const offset = this.view.getUint32(start, true);
    const offset_end = this.view.getUint32(start + 4, true);
    return new DataView(this.view.buffer.slice(offset, offset_end)).getUint8(0);
  }

  getSpvProof () {
    const start = 8;
    const offset = this.view.getUint32(start, true);
    const offset_end = this.view.getUint32(start + 4, true);
    return new Bytes(this.view.buffer.slice(offset, offset_end), {validate: false});
  }

  getCellDepIndexList () {
    const start = 12;
    const offset = this.view.getUint32(start, true);
    const offset_end = this.view.getUint32(start + 4, true);
    return new Bytes(this.view.buffer.slice(offset, offset_end), {validate: false});
  }

  getMerkleProof () {
    const start = 16;
    const offset = this.view.getUint32(start, true);
    const offset_end = this.view.byteLength;
    return new Bytes(this.view.buffer.slice(offset, offset_end), {validate: false});
  }
}

export function SerializeMintTokenWitness (value) {
  const buffers = [];
  const modeView = new DataView(new ArrayBuffer(1));
  modeView.setUint8(0, value.mode);
  buffers.push(modeView.buffer);
  buffers.push(SerializeBytes(value.spv_proof));
  buffers.push(SerializeBytes(value.cell_dep_index_list));
  buffers.push(SerializeBytes(value.merkle_proof));
  return serializeTable(buffers);
}

export class ETHSPVProof {
  constructor (reader, {validate = true} = {}) {
    this.view = new DataView(assertArrayBuffer(reader));
    if (validate) {
      this.validate();
    }
  }

  validate (compatible = false) {
    const offsets = verifyAndExtractOffsets(this.view, 0, true);
    new Uint64(this.view.buffer.slice(offsets[0], offsets[1]), {validate: false}).validate();
    new Uint64(this.view.buffer.slice(offsets[1], offsets[2]), {validate: false}).validate();
    new Bytes(this.view.buffer.slice(offsets[2], offsets[3]), {validate: false}).validate();
    new Bytes(this.view.buffer.slice(offsets[3], offsets[4]), {validate: false}).validate();
    new BytesVec(this.view.buffer.slice(offsets[4], offsets[5]), {validate: false}).validate();
  }

  getLogIndex () {
    const start = 4;
    const offset = this.view.getUint32(start, true);
    const offset_end = this.view.getUint32(start + 4, true);
    return new Uint64(this.view.buffer.slice(offset, offset_end), {validate: false});
  }

  getReceiptIndex () {
    const start = 8;
    const offset = this.view.getUint32(start, true);
    const offset_end = this.view.getUint32(start + 4, true);
    return new Uint64(this.view.buffer.slice(offset, offset_end), {validate: false});
  }

  getReceiptData () {
    const start = 12;
    const offset = this.view.getUint32(start, true);
    const offset_end = this.view.getUint32(start + 4, true);
    return new Bytes(this.view.buffer.slice(offset, offset_end), {validate: false});
  }

  getHeaderData () {
    const start = 16;
    const offset = this.view.getUint32(start, true);
    const offset_end = this.view.getUint32(start + 4, true);
    return new Bytes(this.view.buffer.slice(offset, offset_end), {validate: false});
  }

  getProof () {
    const start = 20;
    const offset = this.view.getUint32(start, true);
    const offset_end = this.view.byteLength;
    return new BytesVec(this.view.buffer.slice(offset, offset_end), {validate: false});
  }
}

export function SerializeETHSPVProof (value) {
  const buffers = [];
  buffers.push(SerializeUint64(value.log_index));
  buffers.push(SerializeUint64(value.receipt_index));
  buffers.push(SerializeBytes(value.receipt_data));
  buffers.push(SerializeBytes(value.header_data));
  buffers.push(SerializeBytesVec(value.proof));
  return serializeTable(buffers);
}

export class Bytes {
  constructor (reader, {validate = true} = {}) {
    this.view = new DataView(assertArrayBuffer(reader));
    if (validate) {
      this.validate();
    }
  }

  validate (compatible = false) {
    if (this.view.byteLength < 4) {
      dataLengthError(this.view.byteLength, '>4');
    }
    const requiredByteLength = this.length() + 4;
    assertDataLength(this.view.byteLength, requiredByteLength);
  }

  raw () {
    return this.view.buffer.slice(4);
  }

  indexAt (i) {
    return this.view.getUint8(4 + i);
  }

  length () {
    return this.view.getUint32(0, true);
  }
}

export function SerializeBytes (value) {
  const item = assertArrayBuffer(value);
  const array = new Uint8Array(4 + item.byteLength);
  (new DataView(array.buffer)).setUint32(0, item.byteLength, true);
  array.set(new Uint8Array(item), 4);
  return array.buffer;
}

export class Byte32 {
  constructor (reader, {validate = true} = {}) {
    this.view = new DataView(assertArrayBuffer(reader));
    if (validate) {
      this.validate();
    }
  }

  validate (compatible = false) {
    assertDataLength(this.view.byteLength, 32);
  }

  indexAt (i) {
    return this.view.getUint8(i);
  }

  raw () {
    return this.view.buffer;
  }

  static size () {
    return 32;
  }
}

export function SerializeByte32 (value) {
  const buffer = assertArrayBuffer(value);
  assertDataLength(buffer.byteLength, 32);
  return buffer;
}

export class ETHAddress {
  constructor (reader, {validate = true} = {}) {
    this.view = new DataView(assertArrayBuffer(reader));
    if (validate) {
      this.validate();
    }
  }

  validate (compatible = false) {
    assertDataLength(this.view.byteLength, 20);
  }

  indexAt (i) {
    return this.view.getUint8(i);
  }

  raw () {
    return this.view.buffer;
  }

  static size () {
    return 20;
  }
}

export function SerializeETHAddress (value) {
  const buffer = assertArrayBuffer(value);
  assertDataLength(buffer.byteLength, 20);
  return buffer;
}

export class BytesVec {
  constructor (reader, {validate = true} = {}) {
    this.view = new DataView(assertArrayBuffer(reader));
    if (validate) {
      this.validate();
    }
  }

  validate (compatible = false) {
    const offsets = verifyAndExtractOffsets(this.view, 0, true);
    for (let i = 0; i < offsets.length - 1; i++) {
      new Bytes(this.view.buffer.slice(offsets[i], offsets[i + 1]), {validate: false}).validate();
    }
  }

  length () {
    if (this.view.byteLength < 8) {
      return 0;
    } else {
      return this.view.getUint32(4, true) / 4 - 1;
    }
  }

  indexAt (i) {
    const start = 4 + i * 4;
    const offset = this.view.getUint32(start, true);
    let offset_end = this.view.byteLength;
    if (i + 1 < this.length()) {
      offset_end = this.view.getUint32(start + 4, true);
    }
    return new Bytes(this.view.buffer.slice(offset, offset_end), {validate: false});
  }
}

export function SerializeBytesVec (value) {
  return serializeTable(value.map(item => SerializeBytes(item)));
}

export class Uint64 {
  constructor (reader, {validate = true} = {}) {
    this.view = new DataView(assertArrayBuffer(reader));
    if (validate) {
      this.validate();
    }
  }

  validate (compatible = false) {
    assertDataLength(this.view.byteLength, 8);
  }

  indexAt (i) {
    return this.view.getUint8(i);
  }

  raw () {
    return this.view.buffer;
  }

  static size () {
    return 8;
  }
}

export function SerializeUint64 (value) {
  const buffer = assertArrayBuffer(value);
  assertDataLength(buffer.byteLength, 8);
  return buffer;
}

export class Uint128 {
  constructor (reader, {validate = true} = {}) {
    this.view = new DataView(assertArrayBuffer(reader));
    if (validate) {
      this.validate();
    }
  }

  validate (compatible = false) {
    assertDataLength(this.view.byteLength, 16);
  }

  indexAt (i) {
    return this.view.getUint8(i);
  }

  raw () {
    return this.view.buffer;
  }

  static size () {
    return 16;
  }
}

export function SerializeUint128 (value) {
  const buffer = assertArrayBuffer(value);
  assertDataLength(buffer.byteLength, 16);
  return buffer;
}


