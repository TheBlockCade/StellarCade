/**
 * useContractWrite — standardized Soroban write execution hook.
 *
 * Provides a reusable, UI-agnostic hook for contract mutation operations.
 * Integrates wallet session, transaction orchestrator, and error mapping.
 *
 * @module hooks/v1/useContractWrite
 */

import { useState, useCallback, useRef } from 'react';
import type {
  ContractWriteOptions,
  ContractWriteResult,
} from '../../types/contracts/write';
import type { AppError } from '../../types/errors';
import { useSorobanClient } from '../useSorobanClient';
import { useWalletSession } from '../useWalletSession';
import { useNetworkGuard } from './useNetworkGuard';
import { mapRpcError } from '../../services/error-mapping';
import { ErrorDomain, ErrorSeverity } from '../../types/errors';
import { executeContractMethod } from '../../utils/v1/contractMethodRegistry';

// ── Types ──────────────────────────────────────────────────────────────────────

type WritePhase = 'idle' | 'loading' | 'success' | 'error';

interface InternalState<TData> {
  phase: WritePhase;
  txHash?: string;
  data?: TData;
  error?: AppError;
  confirmations: number;
}

// ── Hook ───────────────────────────────────────────────────────────────────────

/**
 * Hook for executing Soroban contract write operations.
 *
 * @template TData - The expected return type from the contract method
 *
 * @example
 * ```tsx
 * function PlaceBetButton() {
 *   const { write, isLoading, isSuccess, data, error } = useContractWrite<{ gameId: bigint }>();
 *
 *   const handleBet = async () => {
 *     try {
 *       const result = await write('place_bet', [100_000_000n, 0], {
 *         idempotencyKey: `bet-${Date.now()}`
 *       });
 *       console.log('Game ID:', result.gameId);
 *     } catch (err) {
 *       console.error('Bet failed:', err);
 *     }
 *   };
 *
 *   return (
 *     <button onClick={handleBet} disabled={isLoading}>
 *       {isLoading ? 'Placing bet...' : 'Place Bet'}
 *     </button>
 *   );
 * }
 * ```
 */
export function useContractWrite<TData = unknown>(): ContractWriteResult<TData> {
  const client = useSorobanClient();
  const { state: walletState, meta: walletMeta } = useWalletSession();
  const networkGuard = useNetworkGuard({ network: walletMeta?.network });

  const [state, setState] = useState<InternalState<TData>>({
    phase: 'idle',
    confirmations: 0,
  });

  const idempotencyKeysRef = useRef<Set<string>>(new Set());

  const write = useCallback(
    async (
      method: string,
      params?: any[],
      options?: ContractWriteOptions,
    ): Promise<TData> => {
      // Precondition: wallet connected
      if (walletState !== 'connected') {
        const error: AppError = {
          code: 'WALLET_NOT_CONNECTED',
          domain: ErrorDomain.WALLET,
          severity: ErrorSeverity.USER_ACTIONABLE,
          message: 'Wallet must be connected to execute contract writes',
        };
        setState({ phase: 'error', error, confirmations: 0 });
        throw error;
      }

      // Precondition: supported network
      const networkCheck = networkGuard.assertSupportedNetwork();
      if (!networkCheck.isSupported) {
        const error: AppError = {
          code: 'WALLET_NETWORK_MISMATCH',
          domain: ErrorDomain.WALLET,
          severity: ErrorSeverity.USER_ACTIONABLE,
          message: networkCheck.message,
          context: {
            actual: networkCheck.actual,
            expected: networkCheck.supportedNetworks,
          },
        };
        setState({ phase: 'error', error, confirmations: 0 });
        throw error;
      }

      // Idempotency check
      if (options?.idempotencyKey) {
        if (idempotencyKeysRef.current.has(options.idempotencyKey)) {
          console.warn(
            `[useContractWrite] Duplicate idempotency key detected: ${options.idempotencyKey}`,
          );
        } else {
          idempotencyKeysRef.current.add(options.idempotencyKey);
        }
      }

      // Validation: method required
      if (!method || typeof method !== 'string') {
        const error: AppError = {
          code: 'API_VALIDATION_ERROR',
          domain: ErrorDomain.CONTRACT,
          severity: ErrorSeverity.FATAL,
          message: 'Contract method name is required',
        };
        setState({ phase: 'error', error, confirmations: 0 });
        throw error;
      }

      setState({ phase: 'loading', confirmations: 0 });

      try {
        const result = await executeContractMethod<TData>(
          client,
          method,
          params ?? [],
          options,
        );

        setState({
          phase: 'success',
          txHash: result.txHash,
          data: result.data,
          confirmations: result.confirmations ?? 1,
        });

        return result.data;
      } catch (err: unknown) {
        const mappedError = mapRpcError(err, {
          method,
          params,
          wallet: walletMeta?.publicKey,
        });

        setState({
          phase: 'error',
          error: mappedError,
          confirmations: 0,
        });

        throw mappedError;
      }
    },
    [client, walletState, walletMeta, networkGuard],
  );

  const reset = useCallback(() => {
    setState({ phase: 'idle', confirmations: 0 });
  }, []);

  return {
    phase: state.phase,
    txHash: state.txHash,
    data: state.data,
    error: state.error,
    confirmations: state.confirmations,
    isLoading: state.phase === 'loading',
    isSuccess: state.phase === 'success',
    isError: state.phase === 'error',
    write,
    reset,
  };
}

export default useContractWrite;
