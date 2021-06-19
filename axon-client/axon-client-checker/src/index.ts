import {container, modules, bootstrap} from "./container";
import TaskService from "./modules/services/taskService";
import {logger} from "axon-client-common/src/utils/logger";

export default class AxonCollator {
    #ready = false

    #log = (msg: string) => {
        logger.info(`${msg}`)
    }
    #bootstrap = async () => {
        if (!this.#ready) {
            try {
                await bootstrap()
                this.#ready = true
            } catch (err) {
                logger.error(err)
            }
        }
    }

    public run = async () => {
        // TODO: use decorator to handle bootstrap

        this.#log(`Axon Checker`)

        await this.#bootstrap()

        let taskService = container.get<TaskService>(modules[TaskService.name])
        await taskService.start()
        this.#log('started')
    }
}

new AxonCollator().run()
