import { inject, injectable } from "inversify";
import { modules } from "../../container";
import CKB from "@nervosnetwork/ckb-sdk-core";
import { serializeWitnessArgs } from "@nervosnetwork/ckb-sdk-utils";
import JSONbig from "json-bigint";
import { logger } from "axon-client-common/lib/utils/logger";
import { CKB_LOCK_SCRIPT, BLOCK_MINER_FEE, SELF_PRIVATE_KEY } from "axon-client-common/lib/utils/environment";
import { bigIntToHex } from "axon-client-common/lib/utils/tools";

import { Transformation } from "axon-client-common/lib/modules/models/transformation/interfaces/transformation";

import { CellOutputType } from "axon-client-common/lib/modules/models/cells/interfaces/cell_output_type";
import { CellInputType } from "axon-client-common/lib/modules/models/cells/interfaces/cell_input_type";
import { CellDepType } from "axon-client-common/lib/modules/models/cells/interfaces/cell_dep_type";
import { WitnessInputType } from "axon-client-common/lib/modules/models/witnesses/interfaces/witness_input_type";
import { GenericTransformation } from "axon-client-common/lib/modules/models/transformation/generic_transformation";

import TransactionService from "./transactionService";
import ScanService from "./scanService";

import assert from "assert";

/*
this service compose tx for rpc
 */
@injectable()
export default class OnchainTransactionService implements TransactionService {
  private readonly _ckb: CKB;
  private _isCkbLoadedDeps?: Promise<unknown>;

  private readonly _scanService: ScanService;

  private info(msg: string): void {
    logger.info(`TransactionService: ${msg}`);
  }

  // @ts-expect-error Unused
  // istanbul ignore next
  private error(msg: string): void {
    logger.error(`TransactionService: ${msg}`);
  }

  constructor(@inject(modules.CKBCKB) { ckb }: { ckb: CKB }, @inject(modules.ScanService) scanService: ScanService) {
    this._ckb = ckb;
    this._isCkbLoadedDeps = this._ckb.loadDeps();

    this._scanService = scanService;
  }

  async composeTransaction(xfer: Transformation): Promise<void> {
    if (xfer.skip) {
      return;
    }

    const deps: Array<CKBComponents.CellDep> = [];
    const inputs: Array<CKBComponents.CellInput> = [];
    const outputs: Array<CKBComponents.CellOutput> = [];
    const outputsData: Array<string> = [];
    const witness: Array<CKBComponents.WitnessArgs> = [];

    deps.push(...xfer.toCellDeps());
    inputs.push(...xfer.toCellInput());
    outputs.push(...xfer.toCellOutput());
    outputsData.push(...xfer.toCellOutputData());
    witness.push(...xfer.toWitness());

    // the secp sig shoul be signed at input-0
    const [signedTx, txHash] = await this.composeTxAndSign(deps, inputs, outputs, outputsData, witness, 0n);

    this.info("composed tx: " + JSONbig.stringify(signedTx, null, 2));
    this.info("composed txHash: " + txHash);

    xfer.composedTx = signedTx;
    xfer.composedTxHash = txHash;

    return;
  }

  async composeTransactionFromGeneric<
    DT extends Array<CellDepType>,
    IT extends Array<CellInputType>,
    OT extends Array<CellOutputType>,
    WT extends WitnessInputType | undefined,
  >(xfer: GenericTransformation<DT, IT, OT, WT>): Promise<void> {
    if (xfer.skip) {
      return;
    }

    const deps: Array<CKBComponents.CellDep> = [];
    const inputs: Array<CKBComponents.CellInput> = [];
    const outputs: Array<CKBComponents.CellOutput> = [];
    const outputsData: Array<string> = [];
    const witness: Array<CKBComponents.WitnessArgs> = [];

    deps.push(...xfer.toCellDeps());
    inputs.push(...xfer.toCellInput());
    outputs.push(...xfer.toCellOutput());
    outputsData.push(...xfer.toCellOutputData());
    witness.push(...xfer.toWitness());

    const inputsSize = xfer.cellInputs.map((input) => input.capacity).reduce((a, b) => a + b, 0n);
    assert(xfer.cellOutputs);
    const outputsSize = xfer.cellOutputs.map((output) => output.capacity).reduce((a, b) => a + b, 0n);

    // the secp sig shoul be signed at input-0
    const [signedTx, txHash] = await this.composeTxAndSign(
      deps,
      inputs,
      outputs,
      outputsData,
      witness,
      outputsSize + BLOCK_MINER_FEE - inputsSize,
    );

    this.info("composed tx: " + JSONbig.stringify(signedTx, null, 2));
    this.info("composed txHash: " + txHash);

    xfer.composedTx = signedTx;
    xfer.composedTxHash = txHash;

    return;
  }

  private async composeTxAndSign(
    deps: Array<CKBComponents.CellDep>,
    inputs: Array<CKBComponents.CellInput>,
    outputs: Array<CKBComponents.CellOutput>,
    outputsData: Array<string>,
    witness: Array<CKBComponents.WitnessArgs>,
    unsatisfiedCapacity: bigint,
  ): Promise<[CKBComponents.RawTransaction, string]> {
    await this._isCkbLoadedDeps;
    assert(this._ckb.config.secp256k1Dep);

    deps.push({
      depType: this._ckb.config.secp256k1Dep.depType,
      outPoint: this._ckb.config.secp256k1Dep.outPoint,
    });

    const unspentCells = await this._scanService.scanUnspentCells();
    unspentCells.sort((a, b) => {
      const res = BigInt(a.cell_output.capacity) - BigInt(b.cell_output.capacity);

      if (res > 0) {
        return 1;
      }
      if (res === 0n) {
        return 0;
      }
      return -1;
    });

    let acquiredCapacity = 0n;
    unspentCells.every((cell) => {
      if (acquiredCapacity === unsatisfiedCapacity || acquiredCapacity - unsatisfiedCapacity >= 61n * 100000000n) {
        return false;
      }

      assert(cell.out_point);
      inputs.push({
        previousOutput: {
          txHash: cell.out_point.tx_hash,
          index: cell.out_point.index,
        },
        since: "0x0",
      });

      acquiredCapacity += BigInt(cell.cell_output.capacity);
      return true;
    });

    if (acquiredCapacity > unsatisfiedCapacity) {
      outputs.push({
        capacity: bigIntToHex(acquiredCapacity - unsatisfiedCapacity),
        lock: CKB_LOCK_SCRIPT,
      });
      outputsData.push("0x");
      witness.push({ lock: "0x", inputType: "0x", outputType: "0x" });
    }

    const rawTx: CKBComponents.RawTransaction = {
      version: "0x0",
      headerDeps: [],
      cellDeps: deps,
      inputs,
      witnesses: new Array(inputs.length).fill("0x"),
      outputs,
      outputsData,
    };

    const rawTxWithoutWitness: CKBComponents.RawTransactionToSign = rawTx;

    const txHash = this._ckb.utils.rawTransactionToHash(rawTxWithoutWitness);

    // const secp256k1Witness =  this._ckb.signWitnesses(SELF_PRIVATE_KEY)({
    //     transactionHash: txHash,
    //     witnesses: [{ lock: '', inputType: '', outputType: '' }],
    // })[0]

    //this should be wrong cause we should tell the tool which slot to contain witness
    const secp256k1Witness = this._ckb
      .signWitnesses(SELF_PRIVATE_KEY)({
        transactionHash: txHash,
        witnesses: witness,
      })
      .map((witness): string => {
        if ((witness as CKBComponents.WitnessArgs).lock) {
          return serializeWitnessArgs(witness as CKBComponents.WitnessArgs);
        }
        return witness as string;
      });

    const signedTx: CKBComponents.RawTransaction = {
      ...rawTx,
      witnesses: secp256k1Witness,
    };

    return [signedTx, txHash];
  }
}
