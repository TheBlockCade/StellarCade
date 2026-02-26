import React from 'react';
import type { AsyncStatus } from '../../types/v1/async';

export interface AsyncStateBoundaryProps<T, E = unknown> {
  status: AsyncStatus;
  data?: T | null;
  error?: E | null;
  onRetry?: () => void | Promise<void>;
  renderIdle?: () => React.ReactNode;
  renderLoading?: () => React.ReactNode;
  renderEmpty?: () => React.ReactNode;
  renderError?: (params: { error: E | null | undefined; retry?: () => void | Promise<void> }) => React.ReactNode;
  renderSuccess: (data: T) => React.ReactNode;
  isEmpty?: (data: T) => boolean;
  testId?: string;
}

const VALID_STATUS: readonly AsyncStatus[] = ['idle', 'loading', 'success', 'error'] as const;

export function AsyncStateBoundary<T, E = unknown>({
  status,
  data = null,
  error = null,
  onRetry,
  renderIdle,
  renderLoading,
  renderEmpty,
  renderError,
  renderSuccess,
  isEmpty,
  testId = 'async-state-boundary',
}: AsyncStateBoundaryProps<T, E>) {
  const safeStatus: AsyncStatus = VALID_STATUS.includes(status) ? status : 'idle';

  if (safeStatus === 'idle') {
    return <>{renderIdle?.() ?? null}</>;
  }

  if (safeStatus === 'loading') {
    return <>{renderLoading?.() ?? <div data-testid={`${testId}-loading`}>Loading...</div>}</>;
  }

  if (safeStatus === 'error') {
    if (renderError) {
      return <>{renderError({ error, retry: onRetry })}</>;
    }

    return (
      <div data-testid={`${testId}-error`}>
        <p>Something went wrong.</p>
        {onRetry && (
          <button type="button" onClick={onRetry} data-testid={`${testId}-retry`}>
            Retry
          </button>
        )}
      </div>
    );
  }

  if (data == null) {
    return <>{renderEmpty?.() ?? <div data-testid={`${testId}-empty`}>No data available.</div>}</>;
  }

  if (isEmpty && isEmpty(data)) {
    return <>{renderEmpty?.() ?? <div data-testid={`${testId}-empty`}>No data available.</div>}</>;
  }

  return <>{renderSuccess(data)}</>;
}

export default AsyncStateBoundary;
