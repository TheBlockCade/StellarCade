# ErrorNotice Component - v1

Standardized domain error presenter for Stellarcade frontend. Renders mapped domain errors with safe user-facing messaging, retry/dismiss actions, and optional debug metadata.

## Installation

```typescript
import { ErrorNotice } from '@/components/v1';
// or
import { ErrorNotice } from '@/components/v1/ErrorNotice';
```

## Basic Usage

### Simple Error Display

```typescript
import { ErrorNotice } from '@/components/v1';
import { AppError, ErrorDomain, ErrorSeverity } from '@/types/errors';

function MyComponent() {
  const error: AppError = {
    code: 'WALLET_NOT_CONNECTED',
    domain: ErrorDomain.WALLET,
    severity: ErrorSeverity.USER_ACTIONABLE,
    message: 'Wallet not connected',
  };

  return (
    <ErrorNotice 
      error={error}
      onDismiss={() => setError(null)}
    />
  );
}
```

### Network Error with Retry

```typescript
function NetworkErrorExample() {
  const [error, setError] = useState<AppError | null>(null);
  const [isRetrying, setIsRetrying] = useState(false);

  const handleRetry = async () => {
    setIsRetrying(true);
    try {
      await refetchData();
      setError(null);
    } catch (e) {
      setError(e as AppError);
    } finally {
      setIsRetrying(false);
    }
  };

  return (
    <ErrorNotice
      error={error}
      onRetry={handleRetry}
      onDismiss={() => setError(null)}
      showRetry={true}
      showDismiss={true}
    />
  );
}
```

## Props API

### ErrorNoticeProps

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `error` | `AppError \| unknown` | `undefined` | The error to display (AppError or unknown error) |
| `options` | `ErrorNoticeOptions` | `{}` | Custom options for error normalization |
| `onRetry` | `() => void \| Promise<void>` | `undefined` | Callback when retry button is clicked |
| `onDismiss` | `() => void` | `undefined` | Callback when dismiss button is clicked |
| `showDismiss` | `boolean` | `true` | Whether to show the dismiss button |
| `showRetry` | `boolean` | `true` | Whether to show the retry button (when retryable) |
| `autoDismiss` | `boolean` | `false` | Whether to auto-dismiss retryable errors |
| `className` | `string` | `''` | Custom className for the error notice |
| `testId` | `string` | `'error-notice'` | Test ID for testing |
| `visible` | `boolean` | `true` | Whether component is visible (for controlled usage) |

### ErrorNoticeOptions

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `includeDebug` | `boolean` | `false` | Include debug information in output |
| `customMessage` | `string` | `undefined` | Override default user-friendly message |
| `customAction` | `string` | `undefined` | Override default action suggestion |

## Error Types

The component handles all standard Stellarcade error types:

### RPC Errors
- `RPC_NODE_UNAVAILABLE` - Network temporarily unavailable
- `RPC_CONNECTION_TIMEOUT` - Request timed out
- `RPC_SIMULATION_FAILED` - Transaction simulation failed
- `RPC_TX_REJECTED` - Transaction rejected by network
- `RPC_TX_EXPIRED` - Transaction expired
- `RPC_RESOURCE_LIMIT_EXCEEDED` - Transaction too complex
- `RPC_INVALID_RESPONSE` - Invalid network response

### API Errors
- `API_NETWORK_ERROR` - Cannot connect to servers
- `API_UNAUTHORIZED` - Authentication required
- `API_FORBIDDEN` - Permission denied
- `API_NOT_FOUND` - Resource not found
- `API_VALIDATION_ERROR` - Input validation failed
- `API_RATE_LIMITED` - Too many requests
- `API_SERVER_ERROR` - Internal server error

### Wallet Errors
- `WALLET_NOT_INSTALLED` - Freighter wallet required
- `WALLET_NOT_CONNECTED` - Wallet not connected
- `WALLET_USER_REJECTED` - Transaction cancelled
- `WALLET_NETWORK_MISMATCH` - Wrong network
- `WALLET_INSUFFICIENT_BALANCE` - Insufficient balance
- `WALLET_SIGN_FAILED` - Signing failed

### Contract Errors
- `CONTRACT_NOT_INITIALIZED` - Contract not set up
- `CONTRACT_NOT_AUTHORIZED` - Not authorized
- `CONTRACT_INVALID_AMOUNT` - Invalid amount
- `CONTRACT_INSUFFICIENT_FUNDS` - Insufficient funds
- `CONTRACT_GAME_ALREADY_RESERVED` - Game in progress
- `CONTRACT_RESERVATION_NOT_FOUND` - No active game
- And more...

## Advanced Usage

### Custom Error Messages

```typescript
const error: AppError = {
  code: 'API_VALIDATION_ERROR',
  domain: ErrorDomain.API,
  severity: ErrorSeverity.USER_ACTIONABLE,
  message: 'Validation failed',
};

<ErrorNotice
  error={error}
  options={{
    customMessage: 'Please check your email address and try again',
    customAction: 'Make sure to include @domain.com'
  }}
/>
```

### Debug Mode

```typescript
<ErrorNotice
  error={error}
  options={{
    includeDebug: true // Shows original error and context
  }}
/>
```

### Auto-dismiss for Network Errors

```typescript
<ErrorNotice
  error={networkError}
  autoDismiss={true} // Auto-dismisses timeout errors after 3s
  onDismiss={() => setError(null)}
/>
```

### Controlled Visibility

```typescript
function ControlledExample() {
  const [isVisible, setIsVisible] = useState(true);

  return (
    <ErrorNotice
      error={error}
      visible={isVisible}
      onDismiss={() => setIsVisible(false)}
    />
  );
}
```

## Specialized Components

### NetworkErrorNotice

Pre-configured for network errors with retry functionality:

```typescript
import { NetworkErrorNotice } from '@/components/v1/ErrorNotice';

<NetworkErrorNotice
  onRetry={handleNetworkRetry}
  onDismiss={handleDismiss}
/>
```

### WalletErrorNotice

Pre-configured for wallet connection errors:

```typescript
import { WalletErrorNotice } from '@/components/v1/ErrorNotice';

<WalletErrorNotice
  onDismiss={handleDismiss}
  className="wallet-error"
/>
```

### ValidationErrorNotice

Pre-configured for validation errors:

```typescript
import { ValidationErrorNotice } from '@/components/v1/ErrorNotice';

<ValidationErrorNotice
  onDismiss={handleDismiss}
/>
```

## Styling

The component uses CSS classes based on error severity:

### Base Classes
- `.error-notice` - Base container
- `.error-notice__icon` - Icon container
- `.error-notice__content` - Message container
- `.error-notice__message` - Error message
- `.error-notice__action` - Action suggestion
- `.error-notice__actions` - Button container
- `.error-notice__retry-button` - Retry button
- `.error-notice__dismiss-button` - Dismiss button
- `.error-notice__debug` - Debug information

### Severity Modifiers
- `.error-notice--retryable` - Retryable errors (yellow/orange)
- `.error-notice--user-actionable` - User-actionable errors (blue)
- `.error-notice--fatal` - Fatal errors (red)

### Example CSS

```css
.error-notice {
  display: flex;
  align-items: flex-start;
  gap: 0.75rem;
  padding: 1rem;
  border-radius: 0.5rem;
  background: var(--color-background);
  border: 1px solid var(--color-border);
  margin-bottom: 1rem;
}

.error-notice--retryable {
  border-color: var(--color-warning);
  background: var(--color-warning-background);
}

.error-notice--user-actionable {
  border-color: var(--color-info);
  background: var(--color-info-background);
}

.error-notice--fatal {
  border-color: var(--color-error);
  background: var(--color-error-background);
}

.error-notice__icon {
  flex-shrink: 0;
  width: 1.25rem;
  height: 1.25rem;
  margin-top: 0.125rem;
}

.error-notice__content {
  flex: 1;
  min-width: 0;
}

.error-notice__message {
  font-weight: 500;
  line-height: 1.4;
  color: var(--color-text);
}

.error-notice__action {
  margin-top: 0.25rem;
  font-size: 0.875rem;
  color: var(--color-text-secondary);
}

.error-notice__actions {
  display: flex;
  gap: 0.5rem;
  flex-shrink: 0;
}

.error-notice__retry-button,
.error-notice__dismiss-button {
  padding: 0.375rem 0.75rem;
  border-radius: 0.25rem;
  font-size: 0.875rem;
  border: 1px solid;
  cursor: pointer;
  transition: all 0.15s ease;
}

.error-notice__retry-button {
  background: var(--color-primary);
  border-color: var(--color-primary);
  color: white;
}

.error-notice__dismiss-button {
  background: transparent;
  border-color: var(--color-border);
  color: var(--color-text-secondary);
}

.error-notice__debug {
  margin-top: 0.75rem;
  padding-top: 0.75rem;
  border-top: 1px solid var(--color-border);
}

.error-notice__debug-summary {
  cursor: pointer;
  font-weight: 500;
  color: var(--color-text-secondary);
  margin-bottom: 0.5rem;
}

.error-notice__debug-content {
  font-family: monospace;
  font-size: 0.75rem;
  line-height: 1.4;
}

.error-notice__debug-pre {
  background: var(--color-code-background);
  padding: 0.5rem;
  border-radius: 0.25rem;
  overflow-x: auto;
  white-space: pre-wrap;
}
```

## Testing

The component includes comprehensive test coverage. Example test:

```typescript
import { render, screen, fireEvent } from '@testing-library/react';
import { ErrorNotice } from '@/components/v1';
import { AppError, ErrorDomain, ErrorSeverity } from '@/types/errors';

test('renders error message correctly', () => {
  const error: AppError = {
    code: 'WALLET_NOT_CONNECTED',
    domain: ErrorDomain.WALLET,
    severity: ErrorSeverity.USER_ACTIONABLE,
    message: 'Wallet not connected',
  };

  render(<ErrorNotice error={error} />);
  
  expect(screen.getByRole('alert'))
    .toHaveTextContent('Please connect your wallet to continue.');
});

test('calls onRetry when retry button is clicked', () => {
  const error: AppError = {
    code: 'RPC_NODE_UNAVAILABLE',
    domain: ErrorDomain.RPC,
    severity: ErrorSeverity.RETRYABLE,
    message: 'Network error',
  };

  const onRetry = vi.fn();
  render(<ErrorNotice error={error} onRetry={onRetry} />);
  
  fireEvent.click(screen.getByTestId('error-notice-retry'));
  expect(onRetry).toHaveBeenCalledTimes(1);
});
```

## Best Practices

1. **Always provide onDismiss** for better UX
2. **Use autoDismiss** for transient network errors
3. **Include debug info** only in development environments
4. **Customize messages** for domain-specific context
5. **Test error states** in your components
6. **Use specialized components** when appropriate
7. **Handle loading states** during retry operations

## Migration Guide

### From Basic Error Display

```typescript
// Before
{error && <div className="error">{error.message}</div>}

// After
<ErrorNotice 
  error={error}
  onDismiss={() => setError(null)}
/>
```

### From Custom Error Component

```typescript
// Before
<CustomError 
  error={error}
  onRetry={retry}
  onDismiss={dismiss}
/>

// After
<ErrorNotice
  error={error}
  onRetry={retry}
  onDismiss={dismiss}
  options={{
    customMessage: error.customMessage,
    includeDebug: process.env.NODE_ENV === 'development'
  }}
/>
```

## Dependencies

- React 18+
- TypeScript 4.5+
- Stellarcade error types (`@/types/errors`)
- Error mapping utilities (`@/utils/v1/errorMapper`)

## Accessibility

- Uses `role="alert"` for screen readers
- Provides proper ARIA labels on buttons
- Supports keyboard navigation
- Maintains focus management
- Includes semantic HTML structure
