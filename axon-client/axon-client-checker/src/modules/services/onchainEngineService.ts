import { inject, injectable } from "inversify";
import { modules } from "../../container";
import TransactionService from "./transactionService";
import RpcService from "./rpcService";
import { logger } from "axon-client-common/src/utils/logger";
import { CheckerVoteTransformation } from "axon-client-common/src/modules/models/transformation/checker_vote";
//import { CheckerInfo } from "axon-client-common/src/modules/models/cells/checker_info";
import { CheckerVoteWitness } from "axon-client-common/src/modules/models/witnesses/checker_submit_task_witness";
//import { CheckerSubmitChallengeWitness } from "axon-client-common/src/modules/models/witnesses/checker_submit_challenge_witness";
//import { CheckerPublishChallengeWitness } from "axon-client-common/src/modules/models/witnesses/checker_public_challenge_witness";
import { CheckerSubmitChallengeTransformation } from "axon-client-common/src/modules/models/transformation/checker_submit_challenge";
import { CheckerPublishChallengeTransformation } from "axon-client-common/src/modules/models/transformation/checker_publish_challenge";
import EngineService from "./engineService";
import { DeployCodeTransformation } from "axon-client-common/src/modules/models/transformation/deploy_code_transformation";

@injectable()
export default class OnchainEngineService implements EngineService {
  readonly #transactionService: TransactionService;
  readonly #rpcService: RpcService;

  // @ts-expect-error Unused
  // istanbul ignore next
  private info(msg: string) {
    logger.info(`EngineService: ${msg}`);
  }

  private error(msg: string) {
    logger.error(`EngineService: ${msg}`);
  }

  constructor(
    @inject(modules.TransactionService) transactionService: TransactionService,
    @inject(modules.RpcService) rpcService: RpcService,
  ) {
    this.#transactionService = transactionService;
    this.#rpcService = rpcService;
  }

  checkerVote = async (xfer: CheckerVoteTransformation): Promise<void> => {
    //assume all cell is genuine

    //do state transfer work
    if (xfer.depConfig.checkerTotalCount < xfer.depConfig.checkerThreshold || xfer.inputTask.status !== 0n) {
      return;
    }

    xfer.inputCheckerInfo.status = "Relaying";
    if (xfer.inputTask.mode === 0n) {
      xfer.inputTask.status = 1n;
    } else {
      xfer.inputTask.status = 2n;
    }

    xfer.patternTypeWitness = new CheckerVoteWitness(
      xfer.depConfig.chainId /*xfer.depConfig.chainId*/,
      xfer.inputCheckerInfo.checkerLockArg /*xfer.inputCheckerInfo.chainLockArg*/,
    );

    //compose tx

    await this.#transactionService.composeTransaction(xfer);

    await this.#rpcService.sendTransaction(xfer.composedTx!);
  };

  checkerSubmitChallenge = async (xfer: CheckerSubmitChallengeTransformation): Promise<void> => {
    //assume all cell is genuine

    //do state transfer work
    if (xfer.depConfig.checkerTotalCount < xfer.depConfig.checkerThreshold) {
      return;
    }

    //ToDo fix the logic while witness is fine
    //xfer.inputCheckerInfo.status = CheckerInfo.CHALLENGE_PASSED;
    //xfer.patternTypeWitness = new CheckerSubmitChallengeWitness(xfer.depConfig.chainId, xfer.inputCheckerInfo.checkId);

    //compose tx

    await this.#transactionService.composeTransaction(xfer);

    await this.#rpcService.sendTransaction(xfer.composedTx!);
  };

  checkerPublishChallenge = async (xfer: CheckerPublishChallengeTransformation): Promise<void> => {
    //assume all cell is genuine

    //do state transfer work
    if (xfer.depConfig.checkerTotalCount < xfer.depConfig.checkerThreshold) {
      return;
    }
    //ToDo fix the logic while witness is fine
    //xfer.inputCheckerInfo.mode = CheckerInfo.CHALLENGE_REJECTED;
    //xfer.patternTypeWitness = new CheckerPublishChallengeWitness(xfer.depConfig.chainId, xfer.inputCheckerInfo.checkId);

    //compose tx

    await this.#transactionService.composeTransaction(xfer);

    await this.#rpcService.sendTransaction(xfer.composedTx!);
  };

  async checkerDeployCodeCell(transformation: DeployCodeTransformation): Promise<void> {
    try {
      await this.#transactionService.composeTransactionFromGeneric(transformation);

      await this.#rpcService.sendTransaction(transformation.composedTx!);
    } catch (e: any) {
      this.error(e);
      throw "OnchainEngineService.checkerDeployCodeCell";
    }
    return;
  }
}
