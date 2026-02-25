import { describe, expect, it } from "vitest";

import {
  assertSupportedNetworkBeforeOperation,
  clearNetworkGuardOperationLocks,
  withNetworkGuard,
} from "../../src/services/network-guard-middleware";
import { NetworkGuardError } from "../../src/types/network-guard-middleware";

function provider(network: string) {
  return {
    async getNetwork() {
      return { network };
    },
  };
}

describe("network-guard-middleware", () => {
  it("passes for supported network", async () => {
    const result = await assertSupportedNetworkBeforeOperation({
      walletConnected: true,
      provider: provider("TESTNET"),
      supportedNetworks: ["TESTNET"],
    });

    expect(result.ok).toBe(true);
    expect(result.network.normalized).toBe("TESTNET");
  });

  it("throws typed mismatch error for unsupported network", async () => {
    await expect(
      assertSupportedNetworkBeforeOperation({
        walletConnected: true,
        provider: provider("FUTURENET"),
        supportedNetworks: ["TESTNET"],
      }),
    ).rejects.toMatchObject({
      name: "NetworkGuardError",
      code: "NETWORK_UNSUPPORTED",
    });
  });

  it("blocks missing wallet state", async () => {
    await expect(
      assertSupportedNetworkBeforeOperation({
        walletConnected: false,
        provider: provider("TESTNET"),
      }),
    ).rejects.toMatchObject({
      code: "WALLET_NOT_CONNECTED",
    });
  });

  it("blocks missing contract addresses before side effects", async () => {
    await expect(
      assertSupportedNetworkBeforeOperation({
        walletConnected: true,
        provider: provider("TESTNET"),
        contractAddresses: {
          treasury: "",
        },
      }),
    ).rejects.toMatchObject({
      code: "CONTRACT_ADDRESS_MISSING",
    });
  });

  it("blocks duplicate operation keys", async () => {
    clearNetworkGuardOperationLocks();

    const input = {
      walletConnected: true,
      provider: provider("TESTNET"),
      operationKey: "play-round-1",
    };

    const first = withNetworkGuard(input, async () =>
      new Promise<string>((resolve) => setTimeout(() => resolve("ok"), 25)),
    );

    await expect(withNetworkGuard(input, async () => "second")).rejects.toMatchObject({
      code: "DUPLICATE_OPERATION",
    });

    await expect(first).resolves.toBe("ok");
    clearNetworkGuardOperationLocks();
  });

  it("supports guarded operation execution", async () => {
    const value = await withNetworkGuard(
      {
        walletConnected: true,
        provider: provider("TESTNET"),
        supportedNetworks: ["TESTNET"],
        expectedNetwork: "TESTNET",
      },
      async () => 42,
    );

    expect(value).toBe(42);
  });

  it("maps expected mismatch with remediation hint", async () => {
    await expect(
      assertSupportedNetworkBeforeOperation({
        walletConnected: true,
        provider: provider("TESTNET"),
        expectedNetwork: "PUBLIC",
      }),
    ).rejects.toSatisfy((err) => {
      const e = err as NetworkGuardError;
      return e.code === "NETWORK_MISMATCH" && e.remediationHint.includes("PUBLIC");
    });
  });
});
