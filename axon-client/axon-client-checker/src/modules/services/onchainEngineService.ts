import { inject, injectable } from "inversify";
import { modules } from "../../container";
import TransactionService from "./transactionService";
import RpcService from "./rpcService";
import { logger } from "axon-client-common/lib/utils/logger";
import { CheckerSubmitTaskTransformation } from "axon-client-common/lib/modules/models/transformation/checker_submit_task";
//import { CheckerInfo } from "axon-client-common/lib/modules/models/cells/checker_info";
import { CheckerSubmitTaskWitness } from "axon-client-common/lib/modules/models/witnesses/checker_submit_task_witness";
//import { CheckerSubmitChallengeWitness } from "axon-client-common/lib/modules/models/witnesses/checker_submit_challenge_witness";
//import { CheckerPublishChallengeWitness } from "axon-client-common/lib/modules/models/witnesses/checker_public_challenge_witness";
import { CheckerSubmitChallengeTransformation } from "axon-client-common/lib/modules/models/transformation/checker_submit_challenge";
import { CheckerPublishChallengeTransformation } from "axon-client-common/lib/modules/models/transformation/checker_publish_challenge";
import EngineService from "./engineService";

@injectable()
export default class OnchainEngineService implements EngineService {
  readonly #transactionService: TransactionService;
  readonly #rpcService: RpcService;

  // @ts-expect-error Unused
  // istanbul ignore next
  #info = (outpoint: string, msg: string) => {
    logger.info(`EngineService: ${msg}`);
  };
  // @ts-expect-error Unused
  // istanbul ignore next
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

    xfer.inputCheckerInfo.status = "Relaying";

    //todo change chainId of witness to string from bigint
    xfer.patternTypeWitness = new CheckerSubmitTaskWitness(
      0n /*xfer.depConfig.chainId*/,
      0n /*xfer.inputCheckerInfo.chainId*/,
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
}
