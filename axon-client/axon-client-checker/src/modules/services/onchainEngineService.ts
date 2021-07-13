import { inject, injectable } from "inversify";
import { modules } from "../../container";
import TransactionService from "./transactionService";
import RpcService from "./rpcService";
import { logger } from "axon-client-common/src/utils/logger";
import { CheckerSubmitTaskTransformation } from "axon-client-common/src/modules/models/transformation/checker_submit_task";
import { CheckerInfo } from "axon-client-common/src/modules/models/cells/checker_info";
import { CheckerSubmitTaskWitness } from "axon-client-common/src/modules/models/witnesses/checker_submit_task_witness";
import { CheckSubmitChallengeTransformation } from "axon-client-common/src/modules/models/transformation/checker_submit_challenge";
import { CheckPublishChallengeTransformation } from "axon-client-common/src/modules/models/transformation/checker_publish_challenge";
import EngineService from "./engineService";

@injectable()
export default class OnchainEngineService implements EngineService {
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
    @inject(modules.TransactionService) transactionService: TransactionService,
    @inject(modules.RpcService) rpcService: RpcService,
  ) {
    this.#transactionService = transactionService;
    this.#rpcService = rpcService;
  }

  checkerSubmitTask = async (xfer: CheckerSubmitTaskTransformation): Promise<void> => {
    //assume all cell is genuine

    //do state transfer work
    if (xfer.depConfig.checkerTotalCount < xfer.depConfig.checkerThreshold) {
      return;
    }

    xfer.inputCheckerInfo.mode = CheckerInfo.TASK_PASSED;

    xfer.patternTypeWitness = new CheckerSubmitTaskWitness(xfer.depConfig.chainId, xfer.inputCheckerInfo.checkId);

    //compose tx

    await this.#transactionService.composeTransaction(xfer);

    await this.#rpcService.sendTransaction(xfer.composedTx!);
  };

  checkerSubmitChallenge = async (xfer: CheckSubmitChallengeTransformation): Promise<void> => {
    //assume all cell is genuine

    //do state transfer work
    if (xfer.depConfig.checkerTotalCount < xfer.depConfig.checkerThreshold) {
      xfer.skip = true;
      return;
    }

    xfer.inputCheckerInfo.mode = CheckerInfo.CHALLENGE_PASSED;

    xfer.patternTypeWitness = new CheckerSubmitTaskWitness(xfer.depConfig.chainId, xfer.inputCheckerInfo.checkId);

    xfer.processed = true;
    //compose tx

    await this.#transactionService.composeTransaction(xfer);

    await this.#rpcService.sendTransaction(xfer.composedTx!);
  };

  checkerPublishChallenge = async (xfer: CheckPublishChallengeTransformation): Promise<void> => {
    //assume all cell is genuine

    //do state transfer work
    if (xfer.depConfig.checkerTotalCount < xfer.depConfig.checkerThreshold) {
      xfer.skip = true;
      return;
    }

    xfer.inputCheckerInfo.mode = CheckerInfo.CHALLENGE_REJECTED;

    xfer.patternTypeWitness = new CheckerSubmitTaskWitness(xfer.depConfig.chainId, xfer.inputCheckerInfo.checkId);

    xfer.processed = true;
    //compose tx

    await this.#transactionService.composeTransaction(xfer);

    await this.#rpcService.sendTransaction(xfer.composedTx!);
  };
}
