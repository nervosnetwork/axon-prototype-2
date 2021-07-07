import { CronJob } from "cron";
import { modules } from "../../container";
import { inject, injectable, LazyServiceIdentifer } from "inversify";
import { logger } from "axon-client-common/src/utils/logger";
import ScanService from "./scanService";
import EngineService from "./engineService";
import { CheckerSubmitTaskTransformation } from "axon-client-common/src/modules/models/transformation/checker_submit_task";
import { CheckPublishChallengeTransformation } from "axon-client-common/src/modules/models/transformation/checker_publish_challenge";
import { Task } from "axon-client-common/src/modules/models/cells/task";
import { CheckSubmitChallengeTransformation } from "axon-client-common/src/modules/models/transformation/checker_submit_challenge";

@injectable()
export default class TaskService {
  readonly #scanService: ScanService;
  readonly #engineService: EngineService;

  readonly #schedule = "*/5 * * * * *";

  #cronLock = false;

  #info = (msg: string) => {
    logger.info(`TaskService: ${msg}`);
  };
  #error = (msg: string) => {
    logger.error(`TaskService: ${msg}`);
  };

  constructor(
    @inject(new LazyServiceIdentifer(() => modules[ScanService.name])) scanService: ScanService,
    @inject(new LazyServiceIdentifer(() => modules[EngineService.name])) engineService: EngineService,
  ) {
    this.#scanService = scanService;
    this.#engineService = engineService;
  }

  start = async () => {
    // public task
    new CronJob(this.#schedule, this.wrapperedTask, null, true);
    // submit task or challenge
    // refresh task
    //
  };

  readonly wrapperedTask = async () => {
    if (!this.#cronLock) {
      this.#cronLock = true;
      try {
        this.#info("task job starts: " + new Date());
        await this.task();
        this.#info("task job finishes: " + new Date());
      } catch (e) {
        this.#error("task job error: " + e);
      } finally {
        this.#cronLock = false;
      }
    }
  };

  task = async () => {
    // check submit
    // check publish challenge
    // check submit challenge

    //scan if there are any task belongs to myself
    const code = await this.#scanService.scanCode();
    // @ts-expect-error Unused state
    const state = await this.#scanService.scanSidechainState();
    const globalConfig = await this.#scanService.scanGlobalConfig();
    const config = await this.#scanService.scanSidechainConfig();

    const checkerInfo = await this.#scanService.scanCheckerInfoSelf();
    const tasks = await this.#scanService.scanTask();

    //check out task belonging to self
    const selected = tasks.some((task) => task.chainId != 0n);

    if (selected) {
      //check out task belonging to self
      const task = tasks.filter((task) => task.chainId != 0n)[0];

      // check if submit task or publish challenge
      const challenge = task.mode === Task.CHALLENGE;
      if (challenge) {
        //submit challenge
        const xfer = new CheckSubmitChallengeTransformation(globalConfig, config, code, checkerInfo, task);

        await this.#engineService.checkerSubmitChallenge(xfer);
      } else {
        //submit task, or publish challenge

        const raiseChallenge = true;

        if (raiseChallenge) {
          //publish challenge
          const xfer = new CheckPublishChallengeTransformation(globalConfig, config, code, checkerInfo, task);
          await this.#engineService.checkerPublishChallenge(xfer);
        } else {
          //submit task
          const xfer = new CheckerSubmitTaskTransformation(globalConfig, config, code, checkerInfo, task);

          await this.#engineService.checkerSubmitTask(xfer);
        }
      }
    }
  };
}
