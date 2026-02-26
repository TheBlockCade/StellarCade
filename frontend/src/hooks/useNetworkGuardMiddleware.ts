import { useCallback } from "react";

import type { NetworkGuardInput } from "../types/network-guard-middleware";
import {
  assertSupportedNetworkBeforeOperation,
  withNetworkGuard,
} from "../services/network-guard-middleware";

export interface UseNetworkGuardMiddlewareResult {
  guard: (override?: Partial<NetworkGuardInput>) => Promise<void>;
  runGuarded: <T>(
    operation: () => Promise<T>,
    override?: Partial<NetworkGuardInput>,
  ) => Promise<T>;
}

export function useNetworkGuardMiddleware(
  baseInput: NetworkGuardInput,
): UseNetworkGuardMiddlewareResult {
  const guard = useCallback(
    async (override?: Partial<NetworkGuardInput>) => {
      await assertSupportedNetworkBeforeOperation({ ...baseInput, ...override });
    },
    [baseInput],
  );

  const runGuarded = useCallback(
    async <T>(operation: () => Promise<T>, override?: Partial<NetworkGuardInput>) =>
      withNetworkGuard({ ...baseInput, ...override }, operation),
    [baseInput],
  );

  return {
    guard,
    runGuarded,
  };
}

export default useNetworkGuardMiddleware;
