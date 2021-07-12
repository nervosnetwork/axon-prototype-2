export default interface RpcService {
  sendTransaction(rawTx: CKBComponents.RawTransaction): Promise<boolean>;
}
