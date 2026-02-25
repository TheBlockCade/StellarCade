# useContractWrite Hook

## Overview

`useContractWrite` is a standardized React hook for executing Soroban contract write operations. It provides a reusable, UI-agnostic interface for contract mutations with built-in wallet validation, network guards, error mapping, and idempotency support.

## Features

- ✅ **Wallet Precondition Checks**: Validates wallet connection before execution
- ✅ **Network Validation**: Ensures operations run on supported networks
- ✅ **Idempotency Support**: Prevents duplicate submissions with optional keys
- ✅ **Deterministic State**: Clear phase transitions (idle → loading → success/error)
- ✅ **Error Mapping**: Normalizes RPC/contract errors through error mapper
- ✅ **Type-Safe**: Full TypeScript support with generic return types
- ✅ **UI-Agnostic**: No UI coupling, safe for reuse across components

## Installation

```typescript
import { useContractWrite } from '@/hooks/v1/useContractWrite';
```

## Basic Usage

```typescript
function PlaceBetButton() {
  const { write, isLoading, isSuccess, data, error } = useContractWrite<{ gameId: bigint }>();

  const handleBet = async () => {
    try {
      const result = await write('place_bet', [100_000_000n, 0], {
        idempotencyKey: `bet-${Date.now()}`
      });
      console.log('Game ID:', result.gameId);
    } catch (err) {
      console.error('Bet failed:', err);
    }
  };

  return (
    <button onClick={handleBet} disabled={isLoading}>
      {isLoading ? 'Placing bet...' : 'Place Bet'}
    </button>
  );
}
```

## API Reference

### Hook Signature

```typescript
function useContractWrite<TData = unknown>(): ContractWriteResult<TData>
```

### Return Value

```typescript
interface ContractWriteResult<TData> {
  // State
  phase: 'idle' | 'loading' | 'success' | 'error';
  txHash?: string;
  data?: TData;
  error?: AppError;
  confirmations: number;
  
  // Computed flags
  isLoading: boolean;
  isSuccess: boolean;
  isError: boolean;
  
  // Actions
  write: (method: string, params?: any[], options?: ContractWriteOptions) => Promise<TData>;
  reset: () => void;
}
```

### Write Options

```typescript
interface ContractWriteOptions {
  /** Optional idempotency key to prevent duplicate submissions */
  idempotencyKey?: string;
  
  /** Custom timeout for transaction confirmation (ms) */
  confirmationTimeoutMs?: number;
  
  /** Custom poll interval for status checks (ms) */
  pollIntervalMs?: number;
}
```

## State Transitions

```
idle → loading → success
              ↘ error
```

- **idle**: Initial state, no operation in progress
- **loading**: Write operation executing
- **success**: Operation completed successfully
- **error**: Operation failed with error details

## Precondition Validation

The hook enforces the following preconditions before execution:

1. **Wallet Connected**: Throws `WALLET_NOT_CONNECTED` if wallet is disconnected
2. **Supported Network**: Throws `WALLET_NETWORK_MISMATCH` if on unsupported network
3. **Valid Method**: Throws `API_VALIDATION_ERROR` if method name is invalid

## Error Handling

All errors are normalized through the error mapping service:

```typescript
const { write, error } = useContractWrite();

try {
  await write('method_name', [param1, param2]);
} catch (err: AppError) {
  console.error(err.code);    // Structured error code
  console.error(err.domain);  // Error domain (RPC, CONTRACT, WALLET)
  console.error(err.message); // Human-readable message
}
```

## Idempotency

Prevent duplicate submissions by providing an idempotency key:

```typescript
const { write } = useContractWrite();

// First call - executes normally
await write('transfer', [recipient, amount], {
  idempotencyKey: 'transfer-123'
});

// Second call with same key - warns but still executes
// (Backend should handle actual deduplication)
await write('transfer', [recipient, amount], {
  idempotencyKey: 'transfer-123'
});
```

## Advanced Examples

### With Custom Timeout

```typescript
const { write } = useContractWrite();

await write('complex_operation', [data], {
  confirmationTimeoutMs: 60000, // 60 seconds
  pollIntervalMs: 2000          // Poll every 2 seconds
});
```

### Type-Safe Contract Wrapper

```typescript
interface PrizePoolState {
  totalFunds: bigint;
  reservedFunds: bigint;
}

function usePrizePoolWrite() {
  const base = useContractWrite<PrizePoolState>();
  
  const reserve = (admin: string, gameId: bigint, amount: bigint) =>
    base.write('pool_reserve', [admin, gameId, amount]);
  
  const release = (admin: string, gameId: bigint) =>
    base.write('pool_release', [admin, gameId]);
  
  return {
    ...base,
    reserve,
    release,
  };
}
```

### With Loading UI

```typescript
function TransferButton() {
  const { write, isLoading, isSuccess, error, reset } = useContractWrite();

  const handleTransfer = async () => {
    try {
      await write('transfer', [recipient, amount]);
    } catch (err) {
      // Error is already in state
    }
  };

  if (isSuccess) {
    return (
      <div>
        <p>Transfer successful!</p>
        <button onClick={reset}>Make Another Transfer</button>
      </div>
    );
  }

  return (
    <div>
      <button onClick={handleTransfer} disabled={isLoading}>
        {isLoading ? 'Transferring...' : 'Transfer'}
      </button>
      {error && <p className="error">{error.message}</p>}
    </div>
  );
}
```

## Testing

The hook is fully tested with the following coverage:

- ✅ Initial state validation
- ✅ Precondition checks (wallet, network, params)
- ✅ Idempotency key handling
- ✅ State transitions (idle → loading → success/error)
- ✅ Reset functionality
- ✅ Error mapping integration

See `tests/hooks/v1/useContractWrite.test.ts` for examples.

## Dependencies

- `useSorobanClient`: Contract client instance
- `useWalletSession`: Wallet connection state
- `useNetworkGuard`: Network validation
- `executeContractMethod`: Contract method registry
- `mapRpcError`: Error normalization

## Contract Method Registry

Methods must be registered in `utils/v1/contractMethodRegistry.ts`:

```typescript
export const CONTRACT_METHOD_REGISTRY = {
  'method_name': (client: any, params: any[]) => 
    client.method_name(params[0], params[1]),
  // ... more methods
};
```

## Best Practices

1. **Always handle errors**: Use try/catch or check the `error` state
2. **Use idempotency keys**: For operations that shouldn't be duplicated
3. **Type your data**: Provide generic type for type-safe results
4. **Reset state**: Call `reset()` before new operations if needed
5. **Check loading state**: Disable UI during execution

## Related Hooks

- `useContractRead`: For read-only contract queries
- `useWalletSession`: For wallet connection management
- `useNetworkGuard`: For network validation
- `useTransactionOrchestrator`: For complex multi-step transactions

## License

MIT
