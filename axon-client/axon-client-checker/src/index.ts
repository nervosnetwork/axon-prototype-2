import { container, modules, bootstrap } from "./container";
import DeployService from "./modules/services/deployService";
import TaskService from "./modules/services/taskService";
import { logger } from "axon-client-common/lib/utils/logger";

export default class AxonCollator {
  #ready = false;

  #log = (msg: string) => {
    logger.info(`${msg}`);
  };
  #bootstrap = async () => {
    if (!this.#ready) {
      try {
        await bootstrap();
        this.#ready = true;
      } catch (err) {
        logger.error(err);
      }
    }
  };

  public run = async () => {
    // TODO: use decorator to handle bootstrap

    this.#log(`Axon Checker`);

    await this.#bootstrap();

    const deployService = container.get<DeployService>(modules.DeployService);
    try {
      await deployService.bootstrap();
    } catch (e) {
      logger.error(`Deploy failed: ${e}`);
      return;
    }

    const taskService = container.get<TaskService>(modules.TaskService);
    await taskService.start();
    this.#log("started");
  };
}

new AxonCollator().run().finally(process.exit);
