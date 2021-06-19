import {Container} from 'inversify'
import path from "path";
import {promisify} from "util";
import fs from "fs";
import {logger} from "axon-client-common/src/utils/logger";

export const modules: Record<string, symbol> = {}

export const container = new Container({defaultScope: 'Singleton'})


async function registerModule(modulePath: string) {
    const {default: m} = await import(modulePath)
    modules[m.name] = Symbol(m.name)
    container.bind(modules[m.name]).to(m)
}


export async function bootstrap() {
    // register module
    const modulesDir = path.join(__dirname, 'modules')
    const servicesDir = path.join(modulesDir, 'services')

    for (let injectableDir of [servicesDir]) {
        const injectablePaths = await promisify(fs.readdir)(injectableDir, 'utf8').then(injectableNames =>
            injectableNames.map(injectableName => path.join(injectableDir, injectableName)),
        )
        for (const injectablePath of injectablePaths) {
            try {
                await registerModule(injectablePath)
                logger.info(`inversify: registered module: ${injectablePath}`)
            } catch (e) {
                // we just skip for files don't have injectables :)
            }
        }
    }
}

