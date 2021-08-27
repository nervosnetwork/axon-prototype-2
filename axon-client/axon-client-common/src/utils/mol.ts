import { prepare0xPrefix, remove0xPrefix } from "./tools";
import { BlockSlice, ChainIdList, MolString, SerializeBlockSlice } from "../modules/models/mol/cellData/task";

export type HASH_TYPE = "type" | "code";

export const uint8ToArrayBuffer = (input: bigint): ArrayBuffer => {
  const b = new ArrayBuffer(1);
  const v = new DataView(b);
  v.setUint8(0, Number(input));
  return b;
};

export const arrayBufferToUint8 = (input: ArrayBuffer): bigint => {
  const v = new DataView(input);
  return BigInt(v.getUint8(0));
};

export const uint16ToArrayBuffer = (input: bigint): ArrayBuffer => {
  const b = new ArrayBuffer(2);
  const v = new DataView(b);
  v.setUint16(0, Number(input), true);
  return b;
};

export const arrayBufferToUint16 = (input: ArrayBuffer): bigint => {
  const v = new DataView(input);
  return BigInt(v.getUint16(0, true));
};

export const uint32ToArrayBuffer = (input: bigint): ArrayBuffer => {
  const b = new ArrayBuffer(4);
  const v = new DataView(b);
  v.setUint32(0, Number(input), true);
  return b;
};

export const arrayBufferToUint32 = (input: ArrayBuffer): bigint => {
  const v = new DataView(input);
  return BigInt(v.getUint32(0, true));
};

export const uint64ToArrayBuffer = (input: bigint): ArrayBuffer => {
  const b = new ArrayBuffer(8);
  const v = new DataView(b);
  v.setBigUint64(0, input, true);
  return b;
};

export const arrayBufferToUint64 = (input: ArrayBuffer): bigint => {
  const v = new DataView(input);
  return v.getBigUint64(0, true);
};

export const uint128ToArrayBuffer = (input: bigint): ArrayBuffer => {
  const b = new ArrayBuffer(16);
  const v = new DataView(b);
  v.setBigUint64(0, input & BigInt("0xFFFFFFFFFFFFFFFF"), true);
  v.setBigUint64(8, input >> BigInt(64), true);
  return b;
};

export const arrayBufferToUint128 = (input: ArrayBuffer): bigint => {
  const v = new DataView(input);
  const hi = v.getBigUint64(8, true);
  const lo = v.getBigUint64(0, true);
  return hi << (BigInt(64) + lo);
};

export const bytesXToArrayBuffer = (input: string): ArrayBuffer => {
  input = remove0xPrefix(input);
  const b = Buffer.alloc(input.length / 2);
  b.write(input, "hex");
  return b.buffer;
};

const arrayBufferToBytesInternal = (input: ArrayBuffer): string => {
  const b = Buffer.from(input);
  return prepare0xPrefix(b.toString("hex"));
};

export const arrayBufferToBytesX = arrayBufferToBytesInternal;

export const bytes1ToArrayBuffer = (input: string): ArrayBuffer => {
  input = remove0xPrefix(input);
  const b = Buffer.alloc(1);
  b.write(input, "hex");
  return b.buffer;
};

export const arrayBufferToBytes1 = arrayBufferToBytesInternal;

export const bytes4ToArrayBuffer = (input: string): ArrayBuffer => {
  input = remove0xPrefix(input);
  const b = Buffer.alloc(4);
  b.write(input, "hex");
  return b.buffer;
};

export const arrayBufferToBytes4 = arrayBufferToBytesInternal;

export const bytes16ToArrayBuffer = (input: string): ArrayBuffer => {
  input = remove0xPrefix(input);
  const b = Buffer.alloc(16);
  b.write(input, "hex");
  return b.buffer;
};

export const arrayBufferToBytes16 = arrayBufferToBytesInternal;

export const bytes20ToArrayBuffer = (input: string): ArrayBuffer => {
  input = remove0xPrefix(input);
  const b = Buffer.alloc(20);
  b.write(input, "hex");
  return b.buffer;
};

export const arrayBufferToBytes20 = arrayBufferToBytesInternal;

export const bytes32ToArrayBuffer = (input: string): ArrayBuffer => {
  input = remove0xPrefix(input);
  const b = Buffer.alloc(32);
  b.write(input, "hex");
  return b.buffer;
};

export const arrayBufferToBytes32 = arrayBufferToBytesInternal;

//=======================================

export const blockHeaderToArrayBuffer = bytes32ToArrayBuffer;

export const arrayBufferToBlockHeader = arrayBufferToBytes32;

//=======================================
/*
array BlockHeight [byte; 16];
array Uint128 [byte; 16];
 */
export const blockHeightToArrayBuffer = uint128ToArrayBuffer;

export const arrayBufferToBlockHeight = arrayBufferToUint128;

//=======================================

export const codeHashToArrayBuffer = bytes32ToArrayBuffer;

export const arrayBufferToCodeHash = arrayBufferToBytes32;

//=======================================

export const hashTypeToArrayBuffer = (input: HASH_TYPE): ArrayBuffer => {
  let data = input === "type" ? "0x00" : "0x01";
  data = remove0xPrefix(data);
  const b = Buffer.alloc(1);
  b.write(data, "hex");
  return b.buffer;
};

export const arrayBufferToHashType = (input: ArrayBuffer): HASH_TYPE => {
  const b = Buffer.from(input);
  const data = prepare0xPrefix(b.toString("hex"));
  if (data === "0x00") {
    return "type";
  } else if (data === "0x01") {
    return "code";
  }
  throw new Error("unknown HASH_TYPE encoding");
};

//=======================================

export const merkleHashToArrayBuffer = bytes32ToArrayBuffer;

export const arrayBufferToMerkleHash = arrayBufferToBytes32;

export const publicKeyHashToArrayBuffer = bytes20ToArrayBuffer;

export const arrayBufferToPublicKeyHash = arrayBufferToBytes20;

export const scriptHashToArrayBuffer = bytes32ToArrayBuffer;

export const arrayBufferToScriptHash = arrayBufferToBytes32;

/*
struct BlockSlice {
    from: BlockHeight,
    to: BlockHeight,
}
*/
export const blockSliceToArrayBuffer = ({ from, to }: { from: bigint; to: bigint }): ArrayBuffer => {
  return SerializeBlockSlice({
    from: uint128ToArrayBuffer(from),
    to: uint128ToArrayBuffer(to),
  });
};

export const arrayBufferToBlockSlice = (input: ArrayBuffer): { from: bigint; to: bigint } => {
  const data = new BlockSlice(input);
  const from = arrayBufferToUint128(data.getFrom().raw());
  const to = arrayBufferToUint128(data.getTo().raw());
  return {
    from,
    to,
  };
};

export const chainIdToArrayBuffer = uint32ToArrayBuffer;

export const arrayBufferToChainId = arrayBufferToUint32;

export const chainIdListToWrite = (input: Array<bigint>): Array<ArrayBuffer> => {
  return input.map((chainId) => chainIdToArrayBuffer(chainId));
};

export const readerToChainIdList = (input: ChainIdList): Array<bigint> => {
  const output = [];
  for (let i = 0; i < input.length(); i++) {
    const item = input.indexAt(i);
    output.push(arrayBufferToChainId(item.raw()));
  }
  return output;
};

export const randomSeedToArrayBuffer = bytes32ToArrayBuffer;

export const arrayBufferToRandomSeed = arrayBufferToBytes32;

export const committedHashToArrayBuffer = bytes32ToArrayBuffer;

export const arrayBufferToCommittedHash = arrayBufferToBytes32;

export const molStringToArrayBuffer = bytesXToArrayBuffer;

export const arrayBufferToMolString = (input: ArrayBuffer): string => {
  const t = new MolString(input);
  return arrayBufferToBytesX(t.raw());
};
