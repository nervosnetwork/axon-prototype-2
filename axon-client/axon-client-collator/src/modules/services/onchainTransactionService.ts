import { injectable } from "inversify";
import CKB from "@nervosnetwork/ckb-sdk-core";
import JSONbig from "json-bigint";
import { logger } from "axon-client-common/src/utils/logger";
import { CELL_DEPS, CKB_NODE_URL, SELF_PRIVATE_KEY } from "axon-client-common/src/utils/environment";
import { Transformation } from "axon-client-common/src/modules/models/transformation/interfaces/transformation";
import TransactionService from "./transactionService";

/*
this service compose tx for rpc
 */
@injectable()
export default class OnchainTransactionService implements TransactionService {
  readonly #ckb: CKB;

  #info = (msg: string) => {
    logger.info(`TransactionService: ${msg}`);
  };
  // @ts-expect-error Unused
  #error = (msg: string) => {
    logger.error(`TransactionService: ${msg}`);
  };

  constructor() {
    this.#ckb = new CKB(CKB_NODE_URL);
  }

  composeTransaction = (xfer: Transformation) => {
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
    const [signedTx, txHash] = this.#composeTxAndSign(deps, inputs, outputs, outputsData, witness);

    this.#info("composed tx: " + JSONbig.stringify(signedTx, null, 2));
    this.#info("composed txHash: " + txHash);

    xfer.composedTx = signedTx;
    xfer.composedTxHash = txHash;

    return;
  };

  #composeTxAndSign = (
    deps: Array<CKBComponents.CellDep>,
    inputs: Array<CKBComponents.CellInput>,
    outputs: Array<CKBComponents.CellOutput>,
    outputsData: Array<string>,
    witness: Array<CKBComponents.WitnessArgs>,
  ): [CKBComponents.RawTransaction, string] => {
    deps.push(...CELL_DEPS);

    const rawTx: CKBComponents.RawTransaction = {
      version: "0x0",
      headerDeps: [],
      cellDeps: deps,
      inputs: inputs,
      witnesses: new Array(inputs.length).fill("0x"),
      outputs: outputs,
      outputsData: outputsData,
    };

    const rawTxWithoutWitness: CKBComponents.RawTransactionToSign = rawTx;

    const txHash = this.#ckb.utils.rawTransactionToHash(rawTxWithoutWitness);

    // const secp256k1Witness =  this.#ckb.signWitnesses(SELF_PRIVATE_KEY)({
    //     transactionHash: txHash,
    //     witnesses: [{ lock: '', inputType: '', outputType: '' }],
    // })[0]

    //this should be wrong cause we should tell the tool which slot to contain witness
    const secp256k1Witness = this.#ckb.signWitnesses(SELF_PRIVATE_KEY)({
      transactionHash: txHash,
      witnesses: witness,
    });

    const signedTx: any = {
      ...rawTx,
      //witnesses: [witness, ...rawTransaction.witnesses.slice(1)],
      witnesses: secp256k1Witness,
    };
    return [signedTx, txHash];
  };
}
