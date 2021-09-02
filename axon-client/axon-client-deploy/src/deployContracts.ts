import util from "util";
import fs from "fs";
import { Indexer } from "@ckb-lumos/indexer";
import assert from "assert";

import { GlobalConfig } from "axon-client-common/lib/modules/models/cells/global_config";

import { toHex, addHexPrefix, ckbToShannon, loadCellsFromIndexer, prepare, CONFIGS } from "./common";

const { PRIVATE_KEY, NECESSARY_CELL_CAPACITY, TYPE_ID_CELL_CAPACITY_DIFFERENCE, BIN_PATH_PREFIX, BIN_FILE_NAMES } =
  CONFIGS;

function recordByFileName(globalConfig: GlobalConfig, name: string, hash: string): void {
  switch (name) {
    case "always-success":
      break;
    case "checker-bond-cell-lockscript":
      globalConfig.checkerBondCellLockCodehash = hash;
      globalConfig.checkerBondCellLockHashtype = "code";
      break;
    case "checker-info-cell-typescript":
      globalConfig.checkerInfoCellTypeCodehash = hash;
      globalConfig.checkerInfoCellTypeHashtype = "code";
      break;
    case "code-cell":
      globalConfig.codeCellTypeCodehash = hash;
      globalConfig.codeCellTypeHashtype = "code";
      break;
    case "sidechain-bond-cell-lockscript":
      globalConfig.sidechainBondCellLockCodehash = hash;
      globalConfig.sidechainBondCellLockHashtype = "code";
      break;
    case "sidechain-config-cell-typescript":
      globalConfig.sidechainConfigCellTypeCodehash = hash;
      globalConfig.sidechainConfigCellTypeHashtype = "code";
      break;
    case "sidechain-fee-cell-lockscript":
      globalConfig.sidechainFeeCellLockCodehash = hash;
      globalConfig.sidechainFeeCellLockHashtype = "code";
      break;
    case "sidechain-state-cell-typescript":
      globalConfig.sidechainStateCellTypeCodehash = hash;
      globalConfig.sidechainStateCellTypeHashtype = "code";
      break;
    case "task-cell-typescript":
      globalConfig.taskCellTypeCodehash = hash;
      globalConfig.taskCellTypeHashtype = "code";
      break;
    default:
      assert(false);
      break;
  }
}

export async function deployContracts(indexer: Indexer): Promise<void> {
  const initResult: {
    globalConfigCellInitTransactionHash: string;
    globalConfigCellTypeId: string;
  } = JSON.parse(String(await util.promisify(fs.readFile)("init.json")));

  const { ckb, lock, fromAddress, toAddress } = await prepare();
  assert(ckb.config.secp256k1Dep);
  const { utils } = ckb;

  const typeIdType: CKBComponents.Script = {
    args: initResult.globalConfigCellTypeId,
    codeHash: "0x00000000000000000000000000000000000000000000000000545950455f4944",
    hashType: "type",
  };

  const [unspentCells, typeIdCells, files] = await Promise.all([
    loadCellsFromIndexer(indexer, { lock }),
    loadCellsFromIndexer(indexer, { type: typeIdType }),
    Promise.all(
      BIN_FILE_NAMES.map(async (name) => ({
        name,
        file: await util.promisify(fs.readFile)(BIN_PATH_PREFIX + name),
      })),
    ),
  ]);
  const typeIdCell = typeIdCells[0];
  assert(typeIdCell.type);

  const fileSizesSum = files.map(({ file }) => file.length).reduce((a, b) => a + b, 0);

  const rawTransaction = ckb.generateRawTransaction({
    fromAddress,
    toAddress,
    capacity: ckbToShannon(
      TYPE_ID_CELL_CAPACITY_DIFFERENCE + BigInt(fileSizesSum) + NECESSARY_CELL_CAPACITY * BigInt(files.length),
    ),
    fee: BigInt(1000000),
    safeMode: true,
    cells: unspentCells,
    deps: ckb.config.secp256k1Dep,
  });

  rawTransaction.inputs.push({
    previousOutput: typeIdCells[0].outPoint,
    since: "0x0",
  });

  let remainedCell = null;
  if (rawTransaction.outputs.length > 1) {
    remainedCell = rawTransaction.outputs[1];
  }

  rawTransaction.outputs = [
    {
      capacity: addHexPrefix(toHex(BigInt(typeIdCell.capacity) + ckbToShannon(TYPE_ID_CELL_CAPACITY_DIFFERENCE))),
      lock: typeIdCell.lock,
      type: typeIdCell.type,
    },
  ];

  const globalConfigCell = GlobalConfig.default();
  rawTransaction.outputsData = ["0x"];

  console.log(`index: 0, type id: ${typeIdCell.type.args}`);

  files.forEach(({ name, file }) => {
    const index =
      rawTransaction.outputs.push({
        capacity: addHexPrefix(toHex(ckbToShannon(BigInt(file.length) + NECESSARY_CELL_CAPACITY))),
        lock,
      }) - 1; // Its return value is array's length
    rawTransaction.outputsData.push(utils.bytesToHex(file));

    const blake2b = utils.blake2b(
      32,
      null,
      null,
      new Uint8Array([99, 107, 98, 45, 100, 101, 102, 97, 117, 108, 116, 45, 104, 97, 115, 104]),
    );
    blake2b.update(file);
    const hash = addHexPrefix(String(blake2b.digest("hex")));

    recordByFileName(globalConfigCell, name, hash);

    console.log(`index: ${index}, file name: ${name}, file hash: ${hash}`);
  });

  const globalConfigCellRaw = globalConfigCell.toCellOutputData();
  assert(BigInt(globalConfigCellRaw.length) === TYPE_ID_CELL_CAPACITY_DIFFERENCE + 2n);
  rawTransaction.outputsData[0] = globalConfigCellRaw;
  console.log();

  if (remainedCell) {
    rawTransaction.outputs.push(remainedCell);
    rawTransaction.outputsData.push("0x");
  }

  const signedTx = ckb.signTransaction(PRIVATE_KEY)(rawTransaction);

  const realTxHash = await ckb.rpc.sendTransaction(signedTx);
  console.log(`The real transaction hash is: ${realTxHash}`);

  return;
}
