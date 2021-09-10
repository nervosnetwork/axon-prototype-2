import ScanService from "./scanService";
import EngineService from "./engineService";

import OnchainDeployService from "./onchainDeployService";

import { createMock } from "ts-auto-mock";

const ERROR_MESSAGE = "OnchainDeployService.bootstrap";

describe("OnchainDeployService", () => {
  test("bootstrap should deploy code cell if it's not exist", async () => {
    const scanService = createMock<ScanService>();
    scanService.scanCode = () => Promise.reject();

    const service = new OnchainDeployService(scanService, createMock<EngineService>());
    service.deployCodeCell = jest.fn(() => Promise.resolve());

    await service.bootstrap();

    expect(service.deployCodeCell).toHaveBeenCalledTimes(1);
  });

  test("bootstrap should report error if global config cell is not exist", async () => {
    const scanService = createMock<ScanService>();
    scanService.scanGlobalConfig = () => Promise.reject("Global config cell not found");

    const service = new OnchainDeployService(scanService, createMock<EngineService>());

    await expect(service.bootstrap()).rejects.toBe(ERROR_MESSAGE);
  });

  test("bootstrap should report error if sidechain config cell is not exist", async () => {
    const scanService = createMock<ScanService>();
    scanService.scanSidechainConfig = () => Promise.reject("Sidechain config cell not found");

    const service = new OnchainDeployService(scanService, createMock<EngineService>());

    await expect(service.bootstrap()).rejects.toBe(ERROR_MESSAGE);
  });

  test("bootstrap should does nothing if everything is good", async () => {
    const scanService = createMock<ScanService>();

    const service = new OnchainDeployService(scanService, createMock<EngineService>());
    service.deployCodeCell = jest.fn(() => Promise.resolve());

    await service.bootstrap();

    expect(service.deployCodeCell).toHaveBeenCalledTimes(0);
  });
});
