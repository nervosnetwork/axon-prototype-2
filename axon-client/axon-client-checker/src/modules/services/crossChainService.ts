import JSONbig from 'json-bigint'
import {injectable} from "inversify";
import {logger} from "axon-client-common/src/utils/logger";
// @ts-ignore
const jsonbig = JSONbig({ useNativeBigInt: true,alwaysParseAsBig:true})


@injectable()
export default class CrossChainService {

    // @ts-ignore
    #info = (msg: string) => {
        logger.info(`CrossChainService: ${msg}`)
    }

    // @ts-ignore
    #error = (msg: string) => {
        logger.error(`CrossChainService: ${msg}`)
    }

    constructor(
    ) {
    }

    public getCrossChainInfo = async () :Promise<[bigint, string]>=>{
        return [BigInt(0),'']
    }



}
