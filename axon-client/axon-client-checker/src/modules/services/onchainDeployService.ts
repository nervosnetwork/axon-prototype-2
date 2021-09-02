import { modules } from "../../container";
import { inject, injectable } from "inversify";
import { logger } from "axon-client-common/lib/utils/logger";
import DeployService from "./deployService";
import ScanService from "./scanService";

@injectable()
export default class OnchainDeployService implements DeployService {
  private readonly scanService: ScanService;

  // @ts-expect-error Unused
  // istanbul ignore next
  private info(msg: string) {
    logger.info(`TaskService: ${msg}`);
  }

  private error(msg: string) {
    logger.error(`TaskService: ${msg}`);
  }

  constructor(@inject(modules.ScanService) scanService: ScanService) {
    this.scanService = scanService;
  }

  async deployCodeCell(): Promise<void> {
    // TODO: Deploy code cell
    return;
  }

  async bootstrap(): Promise<void> {
    await Promise.all([
      this.scanService.scanCode().catch(() => this.deployCodeCell()),
      Promise.all([this.scanService.scanGlobalConfig(), this.scanService.scanSidechainConfig()]).catch((e) => {
        this.error(`OnchainDeployService.bootstrap: ${e}`);
        throw "OnchainDeployService.bootstrap";
      }),
    ]);

    return;
  }
}
