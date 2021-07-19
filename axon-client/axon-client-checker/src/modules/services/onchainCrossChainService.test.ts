import "reflect-metadata";

import CrossChainService from "./crossChainService";
import OnchainCrossChainService from "./onchainCrossChainService";

class Context {
  crossChainService: CrossChainService;

  constructor(crossChainService: CrossChainService) {
    this.crossChainService = crossChainService;
  }
}

function prepareContext(): Context {
  return new Context(new OnchainCrossChainService());
}

describe("OnchainCrossChainService", () => {
  test("getCrossChainInfo should success", async () => {
    const context = prepareContext();

    const { latestBlockHeight, latestBlockHash } = await context.crossChainService.getCrossChainInfo();

    expect(latestBlockHeight).toBe(0n);
    expect(latestBlockHash).toBe("");
  });
});
