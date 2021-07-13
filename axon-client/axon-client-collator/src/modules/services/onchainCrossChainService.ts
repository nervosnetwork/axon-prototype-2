import { injectable } from "inversify";
import { logger } from "axon-client-common/src/utils/logger";
import CrossChainService from "./crossChainService";

@injectable()
export default class OnchainCrossChainService implements CrossChainService {
  // @ts-expect-error Unused
  #info = (msg: string) => {
    logger.info(`CrossChainService: ${msg}`);
  };

  // @ts-expect-error Unused
  #error = (msg: string) => {
    logger.error(`CrossChainService: ${msg}`);
  };

  public getCrossChainInfo = async (): Promise<[bigint, string]> => {
    return [BigInt(0), ""];
  };
}
