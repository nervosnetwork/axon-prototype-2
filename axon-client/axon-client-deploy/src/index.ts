import { Indexer } from "@ckb-lumos/indexer";

import { createTypeIdCell } from "./createTypeIdCell";
import { deployContracts } from "./deployContracts";

import { waitIndexerForSync, CONFIGS } from "./common";

const { INDEXER_URL, INDEXER_DB_PATH } = CONFIGS;

async function main(): Promise<void> {
  const indexer = new Indexer(INDEXER_URL, INDEXER_DB_PATH);
  indexer.startForever();

  await waitIndexerForSync(indexer);

  switch (process.argv[2]) {
    case "prepare":
      await createTypeIdCell(indexer);
      break;

    case "deploy":
      await deployContracts(indexer);
      break;

    default:
      console.log("Unknown command");
      break;
  }
}

main().finally(process.exit);
