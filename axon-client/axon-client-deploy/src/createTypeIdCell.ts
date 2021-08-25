import util from "util";
import fs from "fs";
import { Indexer } from "@ckb-lumos/indexer";
import assert from "assert";

import { toHex, addHexPrefix, ckbToShannon, loadCellsFromIndexer, prepare, CONFIGS } from "./common";

const { PRIVATE_KEY, TYPE_ID_CELL_CAPACITY } = CONFIGS;

export async function createTypeIdCell(indexer: Indexer): Promise<void> {
  const result = {
    globalConfigCellInitTransactionHash: "",
    globalConfigCellTypeId: "",
  };

  const { ckb, lock, fromAddress, toAddress } = await prepare();
  assert(ckb.config.secp256k1Dep);
  const { utils } = ckb;

  const unspentCells = await loadCellsFromIndexer(indexer, { lock });

  const rawTransaction = ckb.generateRawTransaction({
    fromAddress,
    toAddress,
    capacity: ckbToShannon(TYPE_ID_CELL_CAPACITY),
    fee: BigInt(1000000),
    safeMode: true,
    cells: unspentCells,
    deps: ckb.config.secp256k1Dep,
  });

  let remainedCell = null;
  if (rawTransaction.outputs.length > 1) {
    remainedCell = rawTransaction.outputs[1];
  }

  const blake2b = utils.blake2b(
    32,
    null,
    null,
    new Uint8Array([99, 107, 98, 45, 100, 101, 102, 97, 117, 108, 116, 45, 104, 97, 115, 104]),
  );

  blake2b.update(utils.hexToBytes(utils.serializeInput(rawTransaction.inputs[0])));
  blake2b.update(utils.hexToBytes(utils.toUint64Le(0n)));
  const digest = blake2b.digest("hex");
  const typeId = addHexPrefix(typeof digest === "string" ? digest : utils.bytesToHex(digest));

  rawTransaction.outputs = [
    {
      capacity: addHexPrefix(toHex(ckbToShannon(TYPE_ID_CELL_CAPACITY))),
      lock,
      type: {
        codeHash: "0x00000000000000000000000000000000000000000000000000545950455f4944",
        hashType: "type",
        args: typeId,
      },
    },
  ];
  rawTransaction.outputsData = ["0x"];

  console.log(`index: 0, type id: ${typeId}`);
  result.globalConfigCellTypeId = typeId;
  console.log();

  if (remainedCell) {
    rawTransaction.outputs.push(remainedCell);
    rawTransaction.outputsData.push("0x");
  }

  const signedTx = ckb.signTransaction(PRIVATE_KEY)(rawTransaction);

  const realTxHash = await ckb.rpc.sendTransaction(signedTx);
  console.log(`The real transaction hash is: ${realTxHash}`);
  result.globalConfigCellInitTransactionHash = realTxHash;

  return await util.promisify(fs.writeFile)("init.json", JSON.stringify(result));
}
