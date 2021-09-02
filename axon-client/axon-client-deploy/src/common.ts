import CKB from "@nervosnetwork/ckb-sdk-core";
import { Indexer, CellCollector } from "@ckb-lumos/indexer";
import assert from "assert";

import { CKB_NODE_URL, SELF_PRIVATE_KEY, INDEXER_URL, INDEXER_DB_PATH } from "axon-client-common/lib/utils/environment";

export const CONFIGS = {
  INDEXER_URL,
  INDEXER_DB_PATH,
  PRIVATE_KEY: SELF_PRIVATE_KEY,
  NECESSARY_CELL_CAPACITY: 61n,
  TYPE_ID_CELL_CAPACITY: 126n,
  TYPE_ID_CELL_CAPACITY_DIFFERENCE: 634n,
  BIN_PATH_PREFIX: "../../axon-ckb-contracts/build/release/",
  BIN_FILE_NAMES: [
    "always-success",
    "checker-bond-cell-lockscript",
    "checker-info-cell-typescript",
    "code-cell",
    "sidechain-bond-cell-lockscript",
    "sidechain-config-cell-typescript",
    "sidechain-fee-cell-lockscript",
    "sidechain-state-cell-typescript",
    "task-cell-typescript",
  ],
};

export function toHex(n: bigint): string {
  if (n < 16) {
    return `0${n.toString(16)}`;
  }

  return n.toString(16);
}

export function addHexPrefix(str: string): string {
  return `0x${str}`;
}

export function ckbToShannon(amount: bigint): bigint {
  return amount * 100000000n;
}

export async function loadCellsFromIndexer(
  indexer: Indexer,
  { lock, type }: { lock?: CKBComponents.Script; type?: CKBComponents.Script },
): Promise<
  Array<{
    data: string;
    lock: CKBComponents.Script;
    type?: CKBComponents.Script;
    capacity: string;
    outPoint: CKBComponents.OutPoint;
  }>
> {
  const collector = new CellCollector(indexer, {
    lock: lock && { code_hash: lock.codeHash, hash_type: lock.hashType, args: lock.args },
    type: type && { code_hash: type.codeHash, hash_type: type.hashType, args: type.args },
  });

  const cells = [];

  for await (const {
    data,
    cell_output: { capacity, lock, type },
    out_point,
  } of collector.collect()) {
    assert(out_point);
    cells.push({
      data,
      lock: { codeHash: lock.code_hash, hashType: lock.hash_type, args: lock.args },
      type: type && { codeHash: type.code_hash, hashType: type.hash_type, args: type.args },
      capacity,
      outPoint: { txHash: out_point.tx_hash, index: out_point.index },
    });
  }

  return cells;
}

export async function waitIndexerForSync(indexer: Indexer): Promise<void> {
  console.log("Waiting for indexer syncing...");
  await indexer.waitForSync();
  console.log("Indexer synced");
  console.log();
}

export async function prepare(): Promise<{
  ckb: CKB;
  publicKey: string;
  publicKeyHash: string;
  lock: CKBComponents.Script;
  fromAddress: string;
  toAddress: string;
}> {
  const ckb = new CKB(CKB_NODE_URL);
  const { utils } = ckb;

  const publicKey = utils.privateKeyToPublicKey(CONFIGS.PRIVATE_KEY);
  console.log(`Public key: ${publicKey}`);

  const publicKeyHash = addHexPrefix(utils.blake160(publicKey, "hex"));
  console.log(`Public key hash: ${publicKeyHash}`);
  console.log();

  await ckb.loadDeps();
  assert(ckb.config.secp256k1Dep);

  const lock = {
    codeHash: ckb.config.secp256k1Dep.codeHash,
    hashType: ckb.config.secp256k1Dep.hashType,
    args: publicKeyHash,
  };

  const fromAddress = utils.pubkeyToAddress(publicKey, {
    prefix: utils.AddressPrefix.Testnet,
  });
  const toAddress = utils.privateKeyToAddress(CONFIGS.PRIVATE_KEY, {
    prefix: utils.AddressPrefix.Testnet,
  });

  return {
    ckb,
    publicKey,
    publicKeyHash,
    lock,
    fromAddress,
    toAddress,
  };
}
