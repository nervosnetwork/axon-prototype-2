import { inject, injectable, LazyServiceIdentifer } from "inversify";
import { SidechainState } from "axon-client-common/src/modules/models/cells/sidechain_state";
import CrossChainService from "./crossChainService";
import { modules } from "../../container";
import { Task } from "axon-client-common/src/modules/models/cells/task";
import { CollatorPublishTaskTransformation } from "axon-client-common/src/modules/models/transformation/collator_publish_task";
import { CollatorPublishTaskWitness } from "axon-client-common/src/modules/models/witnesses/collator_publish_task_witness";
import TransactionService from "./transactionService";
import RpcService from "./rpcService";
import { CollatorSubmitTaskWitness } from "axon-client-common/src/modules/models/witnesses/collator_submit_task_witness";
import { CheckerInfo } from "axon-client-common/src/modules/models/cells/checker_info";
import { CollatorSubmitTaskTransformation } from "axon-client-common/src/modules/models/transformation/collator_submit_task";
import { CollatorSubmitChallengeTransformation } from "axon-client-common/src/modules/models/transformation/collator_submit_challenge";
import { CollatorSubmitChallengeWitness } from "axon-client-common/src/modules/models/witnesses/collator_submit_challenge_witness";
import { CollatorRefreshTaskTransformation } from "axon-client-common/src/modules/models/transformation/collator_refresh_task";
import { CollatorRefreshTaskWitness } from "axon-client-common/src/modules/models/witnesses/collator_refresh_task_witness";
import { logger } from "axon-client-common/src/utils/logger";

@injectable()
export default class EngineService {
  readonly #crossChainService: CrossChainService;
  readonly #transactionService: TransactionService;
  readonly #rpcService: RpcService;

  // @ts-expect-error Unused
  #info = (outpoint: string, msg: string) => {
    logger.info(`EngineService: ${msg}`);
  };
  // @ts-expect-error Unused
  #error = (outpoint: string, msg: string) => {
    logger.error(`EngineService: ${msg}`);
  };

  constructor(
    @inject(new LazyServiceIdentifer(() => modules[CrossChainService.name])) crossChainService: CrossChainService,
    @inject(new LazyServiceIdentifer(() => modules[TransactionService.name])) transactionService: TransactionService,
    @inject(new LazyServiceIdentifer(() => modules[RpcService.name])) rpcService: RpcService,
  ) {
    this.#crossChainService = crossChainService;
    this.#transactionService = transactionService;
    this.#rpcService = rpcService;
  }

  collatorPublishTask = async (xfer: CollatorPublishTaskTransformation) => {
    //assume all cell is genuine

    //get info from crossChainService
    const [latestBlockHeight, latestBlockHash] = await this.#crossChainService.getCrossChainInfo();

    //do state transfer work
    if (xfer.depConfig.checkerTotalCount < xfer.depConfig.checkerThreshold) {
      xfer.skip = true;
      return;
    }

    xfer.inputState.latestBlockHeight = latestBlockHeight;
    xfer.inputState.latestBlockHash = latestBlockHash;
    xfer.inputState.status = SidechainState.STATUS_WAITING_FOR_SUBMIT;

    const bond = 10n;
    xfer.inputBond.unlockSidechainHeight = latestBlockHeight;

    const tasks: Array<Task> = [];

    //should create several task cells
    //now we pick up some checkers,
    //the online version should adopt pseudo-random

    for (let i = 0; i < xfer.depConfig.commitThreshold; i++) {
      const task = Task.default();
      tasks.push(task);
    }

    xfer.outputTask.push(...tasks);

    xfer.patternTypeWitness = new CollatorPublishTaskWitness(xfer.depConfig.chainId, bond);

    xfer.processed = true;
    //compose tx

    await this.#transactionService.composeTransaction(xfer);

    await this.#rpcService.sendTransaction(xfer.composedTx!);
  };

  collatorSubmitTask = async (xfer: CollatorSubmitTaskTransformation) => {
    //assume all cell is genuine

    //do state transfer work
    if (xfer.depConfig.checkerTotalCount < xfer.depConfig.checkerThreshold) {
      xfer.skip = true;
      return;
    }

    for (const checkerInfo of xfer.inputCheckInfos) {
      if (checkerInfo.mode !== CheckerInfo.TASK_PASSED) {
        throw new Error(`~~`);
      }
    }

    xfer.inputState.committedBlockHeight = xfer.inputState.latestBlockHeight;
    xfer.inputState.committedBlockHash = xfer.inputState.latestBlockHash;
    xfer.inputState.status = SidechainState.STATUS_WAITING_FOR_PUBLISH;

    const fee = 10n * BigInt(xfer.inputCheckInfos.length);
    const feePerChecker = 10n;
    xfer.inputFee.museAmount += fee;

    for (const checkerInfo of xfer.inputCheckInfos) {
      checkerInfo.mode = CheckerInfo.CHECKER_IDLE;
    }

    xfer.patternTypeWitness = new CollatorSubmitTaskWitness(xfer.depConfig.chainId, fee, feePerChecker);

    xfer.processed = true;
    //compose tx

    await this.#transactionService.composeTransaction(xfer);

    await this.#rpcService.sendTransaction(xfer.composedTx!);
  };

  collatorSubmitChallenge = async (xfer: CollatorSubmitChallengeTransformation) => {
    //assume all cell is genuine

    //do state transfer work
    if (xfer.inputConfig.checkerTotalCount < xfer.inputConfig.checkerThreshold) {
      xfer.skip = true;
      return;
    }

    for (const checkerInfo of xfer.inputCheckInfos) {
      if (checkerInfo.mode !== CheckerInfo.CHALLENGE_PASSED || checkerInfo.mode !== CheckerInfo.CHALLENGE_REJECTED) {
        throw new Error(`~~`);
      }
    }

    xfer.inputState.committedBlockHeight = xfer.inputState.latestBlockHeight;
    xfer.inputState.committedBlockHash = xfer.inputState.latestBlockHash;
    xfer.inputState.status = SidechainState.STATUS_WAITING_FOR_PUBLISH;

    const fee = 10n * BigInt(xfer.inputCheckInfos.length);
    const feePerChecker = 10n;
    xfer.inputFee.museAmount += fee;

    for (const checkerInfo of xfer.inputCheckInfos) {
      checkerInfo.mode = CheckerInfo.CHECKER_IDLE;
    }

    // do logic to find out who are byzantine
    const punishCheckerBitmap = ``;

    xfer.patternTypeWitness = new CollatorSubmitChallengeWitness(
      xfer.inputConfig.chainId,
      fee,
      feePerChecker,
      punishCheckerBitmap,
    );

    xfer.processed = true;
    //compose tx

    await this.#transactionService.composeTransaction(xfer);

    await this.#rpcService.sendTransaction(xfer.composedTx!);
  };

  refreshTask = async (xfer: CollatorRefreshTaskTransformation) => {
    //assume all cell is genuine

    //do state transfer work
    if (xfer.depConfig.checkerTotalCount < xfer.depConfig.checkerThreshold) {
      xfer.skip = true;
      return;
    }

    for (const task of xfer.inputTasks) {
      //full fill the code
      if (task.refreshInterval > 0) {
        throw new Error(`~~`);
      }
    }

    for (const task of xfer.inputTasks) {
      //full fill the code
      task;
    }

    xfer.patternTypeWitness = new CollatorRefreshTaskWitness(xfer.depConfig.chainId);

    xfer.processed = true;
    //compose tx

    await this.#transactionService.composeTransaction(xfer);

    await this.#rpcService.sendTransaction(xfer.composedTx!);
  };
}
