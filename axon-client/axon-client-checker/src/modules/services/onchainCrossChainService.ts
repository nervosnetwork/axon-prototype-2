import { injectable } from "inversify";
import { logger } from "axon-client-common/lib/utils/logger";
import CrossChainService from "./crossChainService";

@injectable()
export default class OnchainCrossChainService implements CrossChainService {
  // @ts-expect-error Unused
  // istanbul ignore next
  #info = (msg: string) => {
    logger.info(`CrossChainService: ${msg}`);
  };

  // @ts-expect-error Unused
  // istanbul ignore next
  #error = (msg: string) => {
    logger.error(`CrossChainService: ${msg}`);
  };

  public getCrossChainInfo = async (): Promise<{ latestBlockHeight: bigint; latestBlockHash: string }> => {
    return { latestBlockHeight: BigInt(0), latestBlockHash: "" };
  };
}
