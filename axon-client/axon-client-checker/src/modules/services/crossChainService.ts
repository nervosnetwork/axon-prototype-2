export default interface CrossChainService {
  getCrossChainInfo(): Promise<{ latestBlockHeight: bigint; latestBlockHash: string }>;
}
