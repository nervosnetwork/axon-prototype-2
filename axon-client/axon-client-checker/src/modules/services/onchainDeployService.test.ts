import ScanService from "./scanService";

import OnchainDeployService from "./onchainDeployService";

import { createMock } from "ts-auto-mock";

describe("OnchainDeployService", () => {
  test("bootstrap should deploy code cell if it's not exist", async () => {
    const scanService = createMock<ScanService>();
    scanService.scanCode = () => Promise.reject();

    const service = new OnchainDeployService(scanService);
    service.deployCodeCell = jest.fn(() => Promise.resolve());

    await service.bootstrap();

    expect(service.deployCodeCell).toHaveBeenCalledTimes(1);
  });

  test("bootstrap should report error if global config cell is not exist", async () => {
    const errorMessage = "Global config cell not found";
    const scanService = createMock<ScanService>();
    scanService.scanGlobalConfig = () => Promise.reject(errorMessage);

    const service = new OnchainDeployService(scanService);

    await expect(service.bootstrap()).rejects.toBe(errorMessage);
  });

  test("bootstrap should report error if sidechain config cell is not exist", async () => {
    const errorMessage = "Sidechain config cell not found";
    const scanService = createMock<ScanService>();
    scanService.scanSidechainConfig = () => Promise.reject(errorMessage);

    const service = new OnchainDeployService(scanService);

    await expect(service.bootstrap()).rejects.toBe(errorMessage);
  });

  test("bootstrap should does nothing if everything is good", async () => {
    const scanService = createMock<ScanService>();

    const service = new OnchainDeployService(scanService);
    service.deployCodeCell = jest.fn(() => Promise.resolve());

    await service.bootstrap();

    expect(service.deployCodeCell).toHaveBeenCalledTimes(0);
  });
});
