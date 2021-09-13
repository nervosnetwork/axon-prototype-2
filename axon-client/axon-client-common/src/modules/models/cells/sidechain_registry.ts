import { Cell, OutPoint } from "@ckb-lumos/base";
import { SIDECHAIN_REGISTRY_LOCK_SCRIPT, SIDECHAIN_REGISTRY_TYPE_SCRIPT } from "../../../utils/environment";
import { arrayBufferToBigInt, defaultOutPoint, remove0xPrefix, Uint64BigIntToLeHex } from "../../../utils/tools";
import { SidechainRegistryCell } from "../mol/cellData/sidechain_registry";
import { CellInputType } from "./interfaces/cell_input_type";
import { CellOutputType } from "./interfaces/cell_output_type";

export class SidechainRegistry implements CellInputType, CellOutputType {
  capacity: bigint;
  chainIds: Array<bigint>;
  outPoint: OutPoint;

  constructor({ capacity, chainIds, outPoint }: { capacity: bigint; chainIds: Array<bigint>; outPoint: OutPoint }) {
    this.capacity = capacity;
    this.chainIds = chainIds;
    this.outPoint = outPoint;
  }

  static default(): SidechainRegistry {
    return new SidechainRegistry({ capacity: 0n, chainIds: [], outPoint: defaultOutPoint() });
  }

  static validate(cell: Cell): boolean {
    if (!cell.out_point) {
      return false;
    }

    return true;
  }

  static from_cell(cell: Cell): SidechainRegistry | null {
    if (!SidechainRegistry.validate(cell)) {
      return null;
    }

    const capacity = BigInt(cell.cell_output.capacity);
    const cellData = new SidechainRegistryCell(Buffer.from(remove0xPrefix(cell.data), "hex").buffer, {
      validate: true,
    });
    const chainIds: Array<bigint> = [];
    for (let i = 0; i < cellData.getChainIds().length(); i++) {
      const item = cellData.getChainIds().indexAt(i);
      chainIds.push(arrayBufferToBigInt(item.raw()));
    }
    const outPoint = cell.out_point!;
    return new SidechainRegistry({ capacity: capacity, chainIds: chainIds, outPoint: outPoint });
  }

  toCellInput(): CKBComponents.CellInput {
    return {
      previousOutput: {
        txHash: this.outPoint.tx_hash,
        index: this.outPoint.index,
      },
      since: "0x0",
    };
  }
  getOutPoint(): string {
    throw new Error("Method not implemented.");
  }

  toCellOutput(): CKBComponents.CellOutput {
    return {
      capacity: Uint64BigIntToLeHex(this.capacity),
      type: SIDECHAIN_REGISTRY_TYPE_SCRIPT,
      lock: SIDECHAIN_REGISTRY_LOCK_SCRIPT,
    };
  }
  toCellOutputData(): string {
    throw new Error("Method not implemented.");
  }
}
