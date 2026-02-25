/**
 * EmptyStateBlock Component Examples
 * 
 * This file contains example usage patterns for the EmptyStateBlock component.
 * Use these as reference when implementing empty states in your features.
 */

import React from 'react';
import { EmptyStateBlock } from './EmptyStateBlock';
import type { AppError } from '../../types/errors';
import { ErrorDomain, ErrorSeverity } from '../../types/errors';

/**
 * Example 1: Basic variant usage
 */
export const BasicVariantExample: React.FC = () => {
  return (
    <div style={{ display: 'grid', gap: '2rem' }}>
      <EmptyStateBlock variant="list" />
      <EmptyStateBlock variant="search" />
      <EmptyStateBlock variant="transaction" />
      <EmptyStateBlock variant="error" />
      <EmptyStateBlock variant="default" />
    </div>
  );
};

/**
 * Example 2: Custom content
 */
export const CustomContentExample: React.FC = () => {
  return (
    <EmptyStateBlock
      icon="🎮"
      title="No games in your library"
      description="Start playing games to build your collection and track your progress!"
    />
  );
};

/**
 * Example 3: With single action
 */
export const SingleActionExample: React.FC = () => {
  const handleAddItem = () => {
    console.log('Add item clicked');
  };

  return (
    <EmptyStateBlock
      variant="list"
      actions={[
        {
          label: 'Add Your First Item',
          onClick: handleAddItem,
          variant: 'primary',
        },
      ]}
    />
  );
};

/**
 * Example 4: With multiple actions
 */
export const MultipleActionsExample: React.FC = () => {
  const handleClearFilters = () => {
    console.log('Clear filters clicked');
  };

  const handleResetSearch = () => {
    console.log('Reset search clicked');
  };

  const handleBrowseAll = () => {
    console.log('Browse all clicked');
  };

  return (
    <EmptyStateBlock
      variant="search"
      actions={[
        {
          label: 'Clear Filters',
          onClick: handleClearFilters,
          variant: 'primary',
        },
        {
          label: 'Reset Search',
          onClick: handleResetSearch,
        },
        {
          label: 'Browse All',
          onClick: handleBrowseAll,
        },
      ]}
    />
  );
};

/**
 * Example 5: Error state with AppError
 */
export const ErrorStateExample: React.FC = () => {
  const mockError: AppError = {
    code: 'API_NETWORK_ERROR',
    domain: ErrorDomain.API,
    severity: ErrorSeverity.RETRYABLE,
    message: 'Cannot reach the server. Please check your internet connection.',
    retryAfterMs: 3000,
  };

  const handleRetry = async () => {
    console.log('Retrying...');
    // Implement retry logic
  };

  return (
    <EmptyStateBlock
      error={mockError}
      actions={[
        {
          label: 'Try Again',
          onClick: handleRetry,
          variant: 'primary',
        },
      ]}
    />
  );
};

/**
 * Example 6: Override variant defaults
 */
export const OverrideVariantExample: React.FC = () => {
  return (
    <EmptyStateBlock
      variant="list"
      title="Your watchlist is empty"
      description="Add games to your watchlist to get notified about updates and special offers."
      icon="⭐"
    />
  );
};

/**
 * Example 7: Hide description
 */
export const NoDescriptionExample: React.FC = () => {
  return (
    <EmptyStateBlock
      variant="list"
      title="No notifications"
      description={null}
    />
  );
};

/**
 * Example 8: Hide icon
 */
export const NoIconExample: React.FC = () => {
  return (
    <EmptyStateBlock
      variant="list"
      icon={null}
      title="Minimal empty state"
      description="Sometimes less is more."
    />
  );
};

/**
 * Example 9: Custom styling
 */
export const CustomStyledExample: React.FC = () => {
  return (
    <EmptyStateBlock
      variant="list"
      className="custom-empty-state"
      style={{
        backgroundColor: '#f9fafb',
        borderRadius: '8px',
        border: '1px dashed #d1d5db',
      } as React.CSSProperties}
    />
  );
};

/**
 * Example 10: Disabled action
 */
export const DisabledActionExample: React.FC = () => {
  const [isLoading, setIsLoading] = React.useState(false);

  const handleAction = async () => {
    setIsLoading(true);
    // Simulate async operation
    await new Promise((resolve) => setTimeout(resolve, 2000));
    setIsLoading(false);
  };

  return (
    <EmptyStateBlock
      variant="list"
      actions={[
        {
          label: isLoading ? 'Loading...' : 'Load Data',
          onClick: handleAction,
          variant: 'primary',
          disabled: isLoading,
        },
      ]}
    />
  );
};

/**
 * Example 11: Conditional rendering based on state
 */
export const ConditionalExample: React.FC<{
  hasFilters: boolean;
  filterCount: number;
}> = ({ hasFilters, filterCount }) => {
  const handleClearFilters = () => {
    console.log('Clearing filters');
  };

  if (hasFilters) {
    return (
      <EmptyStateBlock
        variant="search"
        title="No results match your filters"
        description={`Try removing some of your ${filterCount} active filters.`}
        actions={[
          {
            label: 'Clear All Filters',
            onClick: handleClearFilters,
            variant: 'primary',
          },
        ]}
      />
    );
  }

  return (
    <EmptyStateBlock
      variant="search"
      title="No results found"
      description="Try a different search term."
    />
  );
};

/**
 * Example 12: With custom React component as icon
 */
export const CustomIconComponentExample: React.FC = () => {
  const CustomIcon = () => (
    <svg
      width="48"
      height="48"
      viewBox="0 0 24 24"
      fill="none"
      stroke="currentColor"
      strokeWidth="2"
    >
      <circle cx="12" cy="12" r="10" />
      <line x1="12" y1="8" x2="12" y2="12" />
      <line x1="12" y1="16" x2="12.01" y2="16" />
    </svg>
  );

  return (
    <EmptyStateBlock
      icon={<CustomIcon />}
      title="Custom Icon Example"
      description="You can use any React component as an icon."
    />
  );
};

/**
 * Example 13: Real-world game library example
 */
export const GameLibraryExample: React.FC = () => {
  const handleBrowseGames = () => {
    console.log('Navigate to games');
  };

  return (
    <EmptyStateBlock
      icon="🎮"
      title="Your game library is empty"
      description="Discover and play exciting blockchain games to start building your collection."
      actions={[
        {
          label: 'Browse Games',
          onClick: handleBrowseGames,
          variant: 'primary',
        },
      ]}
    />
  );
};

/**
 * Example 14: Real-world transaction history example
 */
export const TransactionHistoryExample: React.FC = () => {
  const handlePlayGame = () => {
    console.log('Navigate to games');
  };

  return (
    <EmptyStateBlock
      variant="transaction"
      title="No transactions yet"
      description="Your transaction history will appear here once you start playing games and earning rewards."
      actions={[
        {
          label: 'Play a Game',
          onClick: handlePlayGame,
          variant: 'primary',
        },
      ]}
    />
  );
};

/**
 * Example 15: Real-world error with retry
 */
export const NetworkErrorExample: React.FC = () => {
  const [error, setError] = React.useState<AppError | null>({
    code: 'RPC_NODE_UNAVAILABLE',
    domain: ErrorDomain.RPC,
    severity: ErrorSeverity.RETRYABLE,
    message: 'Soroban RPC node is unreachable. Check your network connection.',
    retryAfterMs: 3000,
  });

  const handleRetry = async () => {
    console.log('Retrying connection...');
    setError(null);
    // Simulate retry
    await new Promise((resolve) => setTimeout(resolve, 1000));
    // For demo, set error again
    setError({
      code: 'RPC_NODE_UNAVAILABLE',
      domain: ErrorDomain.RPC,
      severity: ErrorSeverity.RETRYABLE,
      message: 'Soroban RPC node is unreachable. Check your network connection.',
      retryAfterMs: 3000,
    });
  };

  if (!error) {
    return <div>Loading...</div>;
  }

  return (
    <EmptyStateBlock
      error={error}
      actions={[
        {
          label: 'Retry Connection',
          onClick: handleRetry,
          variant: 'primary',
        },
      ]}
    />
  );
};
