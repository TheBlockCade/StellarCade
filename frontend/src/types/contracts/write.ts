/**
 * Shared types for contract write operations.
 */

import type { AppError } from '../errors';

export interface ContractWriteOptions {
  /** Optional idempotency key to prevent duplicate submissions */
  idempotencyKey?: string;
  /** Custom timeout for transaction confirmation (ms) */
  confirmationTimeoutMs?: number;
  /** Custom poll interval for status checks (ms) */
  pollIntervalMs?: number;
}

export interface ContractWriteState<TData = unknown> {
  /** Current execution phase */
  phase: 'idle' | 'loading' | 'success' | 'error';
  /** Transaction hash if submitted */
  txHash?: string;
  /** Parsed contract result data */
  data?: TData;
  /** Structured error if failed */
  error?: AppError;
  /** Number of ledger confirmations */
  confirmations: number;
  /** Whether the operation is currently executing */
  isLoading: boolean;
  /** Whether the operation succeeded */
  isSuccess: boolean;
  /** Whether the operation failed */
  isError: boolean;
}

export interface ContractWriteResult<TData = unknown> extends ContractWriteState<TData> {
  /** Execute a contract write operation */
  write: (method: string, params?: any[], options?: ContractWriteOptions) => Promise<TData>;
  /** Reset state to idle */
  reset: () => void;
}
