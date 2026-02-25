/**
 * EmptyStateBlock Demo Page
 * 
 * Interactive demo showcasing all features of the EmptyStateBlock component.
 * This can be used as a visual reference or integrated into a component library.
 */

import React, { useState } from 'react';
import { EmptyStateBlock } from './EmptyStateBlock';
import type { EmptyStateVariant } from './EmptyStateBlock.types';
import type { AppError } from '../../types/errors';
import { ErrorDomain, ErrorSeverity } from '../../types/errors';

/**
 * Demo container component
 */
export const EmptyStateBlockDemo: React.FC = () => {
  const [selectedVariant, setSelectedVariant] = useState<EmptyStateVariant>('list');
  const [showActions, setShowActions] = useState(true);
  const [showDescription, setShowDescription] = useState(true);
  const [showIcon, setShowIcon] = useState(true);
  const [customTitle, setCustomTitle] = useState('');
  const [customDescription, setCustomDescription] = useState('');
  const [showError, setShowError] = useState(false);

  const mockError: AppError = {
    code: 'API_NETWORK_ERROR',
    domain: ErrorDomain.API,
    severity: ErrorSeverity.RETRYABLE,
    message: 'Cannot reach the server. Please check your internet connection.',
    retryAfterMs: 3000,
  };

  const handleAction = () => {
    alert('Action clicked!');
  };

  const handleRetry = () => {
    alert('Retry clicked!');
  };

  return (
    <div style={{ padding: '2rem', maxWidth: '1200px', margin: '0 auto' }}>
      <h1 style={{ marginBottom: '2rem' }}>EmptyStateBlock Component Demo</h1>

      {/* Controls */}
      <div
        style={{
          backgroundColor: '#f9fafb',
          padding: '1.5rem',
          borderRadius: '8px',
          marginBottom: '2rem',
        }}
      >
        <h2 style={{ marginTop: 0 }}>Controls</h2>

        <div style={{ display: 'grid', gap: '1rem' }}>
          {/* Variant Selection */}
          <div>
            <label style={{ display: 'block', marginBottom: '0.5rem', fontWeight: 500 }}>
              Variant:
            </label>
            <select
              value={selectedVariant}
              onChange={(e) => setSelectedVariant(e.target.value as EmptyStateVariant)}
              style={{ padding: '0.5rem', borderRadius: '4px', border: '1px solid #d1d5db' }}
            >
              <option value="list">List</option>
              <option value="search">Search</option>
              <option value="transaction">Transaction</option>
              <option value="error">Error</option>
              <option value="default">Default</option>
            </select>
          </div>

          {/* Toggles */}
          <div style={{ display: 'flex', gap: '1rem', flexWrap: 'wrap' }}>
            <label style={{ display: 'flex', alignItems: 'center', gap: '0.5rem' }}>
              <input
                type="checkbox"
                checked={showIcon}
                onChange={(e) => setShowIcon(e.target.checked)}
              />
              Show Icon
            </label>
            <label style={{ display: 'flex', alignItems: 'center', gap: '0.5rem' }}>
              <input
                type="checkbox"
                checked={showDescription}
                onChange={(e) => setShowDescription(e.target.checked)}
              />
              Show Description
            </label>
            <label style={{ display: 'flex', alignItems: 'center', gap: '0.5rem' }}>
              <input
                type="checkbox"
                checked={showActions}
                onChange={(e) => setShowActions(e.target.checked)}
              />
              Show Actions
            </label>
            <label style={{ display: 'flex', alignItems: 'center', gap: '0.5rem' }}>
              <input
                type="checkbox"
                checked={showError}
                onChange={(e) => setShowError(e.target.checked)}
              />
              Use Error Prop
            </label>
          </div>

          {/* Custom Title */}
          <div>
            <label style={{ display: 'block', marginBottom: '0.5rem', fontWeight: 500 }}>
              Custom Title (optional):
            </label>
            <input
              type="text"
              value={customTitle}
              onChange={(e) => setCustomTitle(e.target.value)}
              placeholder="Leave empty to use variant default"
              style={{
                width: '100%',
                padding: '0.5rem',
                borderRadius: '4px',
                border: '1px solid #d1d5db',
              }}
            />
          </div>

          {/* Custom Description */}
          <div>
            <label style={{ display: 'block', marginBottom: '0.5rem', fontWeight: 500 }}>
              Custom Description (optional):
            </label>
            <textarea
              value={customDescription}
              onChange={(e) => setCustomDescription(e.target.value)}
              placeholder="Leave empty to use variant default"
              rows={2}
              style={{
                width: '100%',
                padding: '0.5rem',
                borderRadius: '4px',
                border: '1px solid #d1d5db',
                fontFamily: 'inherit',
              }}
            />
          </div>
        </div>
      </div>

      {/* Preview */}
      <div
        style={{
          border: '2px dashed #d1d5db',
          borderRadius: '8px',
          padding: '2rem',
          backgroundColor: 'white',
        }}
      >
        <h2 style={{ marginTop: 0, marginBottom: '1.5rem' }}>Preview</h2>
        <EmptyStateBlock
          variant={selectedVariant}
          icon={showIcon ? undefined : null}
          title={customTitle || undefined}
          description={
            showDescription ? (customDescription || undefined) : null
          }
          actions={
            showActions
              ? [
                  {
                    label: 'Primary Action',
                    onClick: handleAction,
                    variant: 'primary',
                  },
                  {
                    label: 'Secondary Action',
                    onClick: handleAction,
                  },
                ]
              : undefined
          }
          error={showError ? mockError : undefined}
        />
      </div>

      {/* Code Example */}
      <div
        style={{
          marginTop: '2rem',
          backgroundColor: '#1e293b',
          color: '#e2e8f0',
          padding: '1.5rem',
          borderRadius: '8px',
          overflow: 'auto',
        }}
      >
        <h3 style={{ marginTop: 0, color: '#e2e8f0' }}>Code</h3>
        <pre style={{ margin: 0, fontSize: '0.875rem' }}>
          <code>{generateCodeExample()}</code>
        </pre>
      </div>

      {/* All Variants Showcase */}
      <div style={{ marginTop: '3rem' }}>
        <h2>All Variants</h2>
        <div style={{ display: 'grid', gap: '2rem', gridTemplateColumns: 'repeat(auto-fit, minmax(300px, 1fr))' }}>
          <div style={{ border: '1px solid #e5e7eb', borderRadius: '8px', padding: '1rem' }}>
            <h3 style={{ marginTop: 0, fontSize: '1rem', color: '#6b7280' }}>List</h3>
            <EmptyStateBlock variant="list" />
          </div>
          <div style={{ border: '1px solid #e5e7eb', borderRadius: '8px', padding: '1rem' }}>
            <h3 style={{ marginTop: 0, fontSize: '1rem', color: '#6b7280' }}>Search</h3>
            <EmptyStateBlock variant="search" />
          </div>
          <div style={{ border: '1px solid #e5e7eb', borderRadius: '8px', padding: '1rem' }}>
            <h3 style={{ marginTop: 0, fontSize: '1rem', color: '#6b7280' }}>Transaction</h3>
            <EmptyStateBlock variant="transaction" />
          </div>
          <div style={{ border: '1px solid #e5e7eb', borderRadius: '8px', padding: '1rem' }}>
            <h3 style={{ marginTop: 0, fontSize: '1rem', color: '#6b7280' }}>Error</h3>
            <EmptyStateBlock variant="error" />
          </div>
          <div style={{ border: '1px solid #e5e7eb', borderRadius: '8px', padding: '1rem' }}>
            <h3 style={{ marginTop: 0, fontSize: '1rem', color: '#6b7280' }}>Default</h3>
            <EmptyStateBlock variant="default" />
          </div>
        </div>
      </div>

      {/* Real-world Examples */}
      <div style={{ marginTop: '3rem' }}>
        <h2>Real-world Examples</h2>
        
        <div style={{ display: 'grid', gap: '2rem' }}>
          {/* Game Library */}
          <div style={{ border: '1px solid #e5e7eb', borderRadius: '8px', padding: '1.5rem' }}>
            <h3 style={{ marginTop: 0 }}>Game Library</h3>
            <EmptyStateBlock
              icon="🎮"
              title="Your game library is empty"
              description="Discover and play exciting blockchain games to start building your collection."
              actions={[
                {
                  label: 'Browse Games',
                  onClick: () => alert('Navigate to games'),
                  variant: 'primary',
                },
              ]}
            />
          </div>

          {/* Network Error */}
          <div style={{ border: '1px solid #e5e7eb', borderRadius: '8px', padding: '1.5rem' }}>
            <h3 style={{ marginTop: 0 }}>Network Error</h3>
            <EmptyStateBlock
              error={mockError}
              actions={[
                {
                  label: 'Retry Connection',
                  onClick: handleRetry,
                  variant: 'primary',
                },
              ]}
            />
          </div>

          {/* Search with Filters */}
          <div style={{ border: '1px solid #e5e7eb', borderRadius: '8px', padding: '1.5rem' }}>
            <h3 style={{ marginTop: 0 }}>Search with Filters</h3>
            <EmptyStateBlock
              variant="search"
              title="No results match your filters"
              description="Try removing some filters or adjusting your search terms."
              actions={[
                {
                  label: 'Clear All Filters',
                  onClick: () => alert('Clear filters'),
                  variant: 'primary',
                },
                {
                  label: 'Reset Search',
                  onClick: () => alert('Reset search'),
                },
              ]}
            />
          </div>
        </div>
      </div>
    </div>
  );

  function generateCodeExample(): string {
    const props: string[] = [];
    
    if (selectedVariant !== 'default') {
      props.push(`variant="${selectedVariant}"`);
    }
    
    if (!showIcon) {
      props.push('icon={null}');
    }
    
    if (customTitle) {
      props.push(`title="${customTitle}"`);
    }
    
    if (!showDescription) {
      props.push('description={null}');
    } else if (customDescription) {
      props.push(`description="${customDescription}"`);
    }
    
    if (showActions) {
      props.push(`actions={[
    { label: 'Primary Action', onClick: handleAction, variant: 'primary' },
    { label: 'Secondary Action', onClick: handleAction }
  ]}`);
    }
    
    if (showError) {
      props.push('error={appError}');
    }
    
    if (props.length === 0) {
      return '<EmptyStateBlock />';
    }
    
    return `<EmptyStateBlock\n  ${props.join('\n  ')}\n/>`;
  }
};

export default EmptyStateBlockDemo;
