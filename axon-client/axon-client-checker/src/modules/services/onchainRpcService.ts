import { inject, injectable } from "inversify";
import { modules } from "../../container";
import Rpc from "@nervosnetwork/ckb-sdk-rpc";
import JSONbig from "json-bigint";
import { logger } from "axon-client-common/lib/utils/logger";
import RpcService from "./rpcService";

@injectable()
export default class OnchainRpcService implements RpcService {
  #client: Rpc;

  // @ts-expect-error Unused
  // istanbul ignore next
  private info(msg: string): void {
    logger.info(`RpcService: ${msg}`);
  }

  private error(msg: string): void {
    logger.error(`RpcService: ${msg}`);
  }

  constructor(@inject(modules.CKBRpc) { rpc }: { rpc: Rpc }) {
    this.#client = rpc;
  }

  sendTransaction = async (rawTx: CKBComponents.RawTransaction): Promise<boolean> => {
    try {
      //this.#info('sendTransaction : ' + JSONbig.stringify(rawTx, null, 2))
      await this.#client.sendTransaction(rawTx);
      return true;
    } catch (e) {
      this.error("sendTransaction error: " + e);
      this.error("rawTx: " + JSONbig.stringify(rawTx, null, 2));
      return false;
    }
  };

  // @ts-expect-error Unused
  // istanbul ignore next
  private toArrayBuffer(buf: Buffer): ArrayBuffer {
    const ab = new ArrayBuffer(buf.length);
    const view = new Uint8Array(ab);
    for (let i = 0; i < buf.length; ++i) {
      view[i] = buf[i];
    }
    return ab;
  }

  // @ts-expect-error Unused
  // istanbul ignore next
  private toBuffer(arrayBuffer: ArrayBuffer): Buffer {
    const b = Buffer.alloc(arrayBuffer.byteLength);
    const view = new Uint8Array(arrayBuffer);

    for (let i = 0; i < b.length; ++i) {
      b[i] = view[i];
    }
    return b;
  }
}
