import {inject, injectable, LazyServiceIdentifer} from 'inversify'
// @ts-ignore
import sqrt from 'bigint-isqrt'
import CrossChainService from "./crossChainService";
import {modules} from "../../container";
import TransactionService from "./transactionService";
import RpcService from "./rpcService";import {logger} from "axon-client-common/src/utils/logger";
import {CheckerSubmitTaskTransformation} from "axon-client-common/src/modules/models/transformation/checker_submit_task";
import {CheckerInfo} from "axon-client-common/src/modules/models/cells/checker_info";
import {CheckerSubmitTaskWitness} from "axon-client-common/src/modules/models/witnesses/checker_submit_task_witness";
import {CheckSubmitChallengeTransformation} from "axon-client-common/src/modules/models/transformation/checker_submit_challenge";
import {CheckPublishChallengeTransformation} from "axon-client-common/src/modules/models/transformation/checker_publish_challenge";

@injectable()
export default class EngineService {

    // @ts-ignore
    readonly #crossChainService: CrossChainService
    readonly #transactionService: TransactionService
    readonly #rpcService: RpcService

    // @ts-ignore
    #info = (outpoint: string, msg: string) => {
        logger.info(`EngineService: ${msg}`)
    }
    // @ts-ignore
    #error = (outpoint: string, msg: string) => {
        logger.error(`EngineService: ${msg}`)
    }

    constructor(
        @inject(new LazyServiceIdentifer(() => modules[CrossChainService.name])) crossChainService: CrossChainService,
        @inject(new LazyServiceIdentifer(() => modules[TransactionService.name])) transactionService: TransactionService,
        @inject(new LazyServiceIdentifer(() => modules[RpcService.name])) rpcService: RpcService,
    ) {
        this.#crossChainService = crossChainService
        this.#transactionService = transactionService
        this.#rpcService = rpcService

    }


    checkerSubmitTask = async (xfer: CheckerSubmitTaskTransformation,) => {

        //assume all cell is genuine

        //do state transfer work
        if (xfer.depConfig.checkerTotalCount < xfer.depConfig.checkerThreshold) {
            xfer.skip = true
            return
        }

        xfer.inputCheckerInfo.mode = CheckerInfo.TASK_PASSED



        xfer.patternTypeWitness = new CheckerSubmitTaskWitness(xfer.depConfig.chainId, xfer.inputCheckerInfo.checkId)

        xfer.processed = true
        //compose tx

        await this.#transactionService.composeTransaction(xfer)

        await this.#rpcService.sendTransaction(xfer.composedTx!)
    }

    checkerSubmitChallenge = async (xfer: CheckSubmitChallengeTransformation,) => {

        //assume all cell is genuine

        //do state transfer work
        if (xfer.depConfig.checkerTotalCount < xfer.depConfig.checkerThreshold) {
            xfer.skip = true
            return
        }

        xfer.inputCheckerInfo.mode = CheckerInfo.CHALLENGE_PASSED


        xfer.patternTypeWitness = new CheckerSubmitTaskWitness(xfer.depConfig.chainId, xfer.inputCheckerInfo.checkId)

        xfer.processed = true
        //compose tx

        await this.#transactionService.composeTransaction(xfer)

        await this.#rpcService.sendTransaction(xfer.composedTx!)
    }

    checkerPublishChallenge = async (xfer: CheckPublishChallengeTransformation,) => {

        //assume all cell is genuine

        //do state transfer work
        if (xfer.depConfig.checkerTotalCount < xfer.depConfig.checkerThreshold) {
            xfer.skip = true
            return
        }

        xfer.inputCheckerInfo.mode = CheckerInfo.CHALLENGE_REJECTED


        xfer.patternTypeWitness = new CheckerSubmitTaskWitness(xfer.depConfig.chainId, xfer.inputCheckerInfo.checkId)

        xfer.processed = true
        //compose tx

        await this.#transactionService.composeTransaction(xfer)

        await this.#rpcService.sendTransaction(xfer.composedTx!)
    }
}
