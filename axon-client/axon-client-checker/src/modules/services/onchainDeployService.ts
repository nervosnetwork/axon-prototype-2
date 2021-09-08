import { modules } from "../../container";
import { inject, injectable } from "inversify";

import DeployService from "./deployService";
import ScanService from "./scanService";
import EngineService from "./engineService";

import { SECP256K1_ARG } from "axon-client-common/lib/utils/environment";
import { logger } from "axon-client-common/lib/utils/logger";
import { Code } from "axon-client-common/lib/modules/models/cells/code";
import { DeployCodeTransformation } from "axon-client-common/lib/modules/models/transformation/deploy_code_transformation";

@injectable()
export default class OnchainDeployService implements DeployService {
  private readonly scanService: ScanService;
  private readonly engineService: EngineService;

  // @ts-expect-error Unused
  // istanbul ignore next
  private info(msg: string) {
    logger.info(`TaskService: ${msg}`);
  }

  private error(msg: string) {
    logger.error(`TaskService: ${msg}`);
  }

  constructor(
    @inject(modules.ScanService) scanService: ScanService,
    @inject(modules.EngineService) engineService: EngineService,
  ) {
    this.scanService = scanService;
    this.engineService = engineService;
  }

  async deployCodeCell(): Promise<void> {
    const transformation = new DeployCodeTransformation({ cellOutputs: [new Code(1000n * 100000000n, SECP256K1_ARG)] });

    await this.engineService.checkerDeployCodeCell(transformation);

    return;
  }

  async bootstrap(): Promise<void> {
    await this.scanService.scanCode().catch(() => this.deployCodeCell());

    await Promise.all([
      Promise.all([this.scanService.scanGlobalConfig(), this.scanService.scanSidechainConfig()]).catch((e) => {
        this.error(`OnchainDeployService.bootstrap: ${e}`);
        throw "OnchainDeployService.bootstrap";
      }),
    ]);

    return;
  }
}
