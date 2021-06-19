import { injectable } from 'inversify'
import Rpc from '@nervosnetwork/ckb-sdk-rpc'
import JSONbig from 'json-bigint'
import {logger} from "axon-client-common/src/utils/logger";
import {CKB_NODE_URL} from "axon-client-common/src/utils/environment";
@injectable()
export default class RpcService {
    #client: Rpc

    // @ts-ignore
    #info = (msg: string) => {
        logger.info(`RpcService: ${msg}`)
    }
    // @ts-ignore
    #error = (msg: string) => {
        logger.error(`RpcService: ${msg}`)
    }

    constructor() {
        this.#client = new Rpc(CKB_NODE_URL)
    }

    sendTransaction = async (rawTx: CKBComponents.RawTransaction): Promise<boolean> => {
        try {
            //this.#info('sendTransaction : ' + JSONbig.stringify(rawTx, null, 2))
            await this.#client.sendTransaction(rawTx)
            return true
        } catch (e) {
            this.#error('sendTransaction error: ' + e)
            this.#error('rawTx: '+JSONbig.stringify(rawTx,null,2))
            return false
        }
    }

    // @ts-ignore
    private toArrayBuffer = (buf: Buffer): ArrayBuffer => {
        let ab = new ArrayBuffer(buf.length)
        let view = new Uint8Array(ab)
        for (let i = 0; i < buf.length; ++i) {
            view[i] = buf[i]
        }
        return ab
    }

    // @ts-ignore
    private toBuffer = (arrayBuffer: ArrayBuffer): Buffer => {
        let b = Buffer.alloc(arrayBuffer.byteLength)
        let view = new Uint8Array(arrayBuffer)

        for (let i = 0; i < b.length; ++i) {
            b[i] = view[i]
        }
        return b
    }
}
