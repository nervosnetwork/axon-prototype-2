import "reflect-metadata";

import Rpc from "@nervosnetwork/ckb-sdk-rpc";

import OnchainRpcService from "./onchainRpcService";

import { createMock } from "ts-auto-mock";

describe("OnchainRpcService", () => {
  test("sendTransaction should failed if rpc failed", async () => {
    const mockRpc = createMock<Rpc>();
    mockRpc.sendTransaction = jest.fn(() => {
      throw "Error";
    });
    const rpcService = new OnchainRpcService(mockRpc);

    const mockTransaction = createMock<CKBComponents.RawTransaction>();

    expect(await rpcService.sendTransaction(mockTransaction)).toBeFalsy();

    const sendTransaction = mockRpc.sendTransaction as jest.Mock;
    expect(sendTransaction).toHaveBeenCalledTimes(1);
    expect(sendTransaction).toHaveBeenCalledWith(mockTransaction);
  });

  test("sendTransaction should success", async () => {
    const mockRpc = createMock<Rpc>();
    const rpcService = new OnchainRpcService(mockRpc);

    const mockTransaction = createMock<CKBComponents.RawTransaction>();

    expect(await rpcService.sendTransaction(mockTransaction)).toBeTruthy();

    const sendTransaction = mockRpc.sendTransaction as jest.Mock;
    expect(sendTransaction).toHaveBeenCalledTimes(1);
    expect(sendTransaction).toHaveBeenCalledWith(mockTransaction);
  });
});
