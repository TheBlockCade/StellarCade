# EmptyStateBlock Migration Guide

This guide helps you migrate existing empty state implementations to use the new EmptyStateBlock component.

## Why Migrate?

The EmptyStateBlock component provides:
- ✅ Consistent UI/UX across the application
- ✅ Built-in accessibility features
- ✅ XSS protection and input sanitization
- ✅ Error integration with AppError system
- ✅ Responsive design out of the box
- ✅ Reduced code duplication

## Before and After Examples

### Example 1: Basic Empty List

**Before:**
```tsx
function GameList({ games }: { games: Game[] }) {
  if (games.length === 0) {
    return (
      <div className="empty-state">
        <div className="icon">📋</div>
        <h2>No games yet</h2>
        <p>Games will appear here once they are added.</p>
      </div>
    );
  }
  
  return <div>{/* render games */}</div>;
}
```

**After:**
```tsx
import { EmptyStateBlock } from '@/components/v1';

function GameList({ games }: { games: Game[] }) {
  if (games.length === 0) {
    return <EmptyStateBlock variant="list" />;
  }
  
  return <div>{/* render games */}</div>;
}
```

### Example 2: Empty State with Action

**Before:**
```tsx
function SearchResults({ results, onClearFilters }: Props) {
  if (results.length === 0) {
    return (
      <div className="empty-state">
        <div className="icon">🔍</div>
        <h2>No results found</h2>
        <p>Try adjusting your search terms or filters.</p>
        <button onClick={onClearFilters} className="btn-primary">
          Clear Filters
        </button>
      </div>
    );
  }
  
  return <div>{/* render results */}</div>;
}
```

**After:**
```tsx
import { EmptyStateBlock } from '@/components/v1';

function SearchResults({ results, onClearFilters }: Props) {
  if (results.length === 0) {
    return (
      <EmptyStateBlock
        variant="search"
        actions={[
          { label: 'Clear Filters', onClick: onClearFilters, variant: 'primary' }
        ]}
      />
    );
  }
  
  return <div>{/* render results */}</div>;
}
```

### Example 3: Error State

**Before:**
```tsx
function DataView({ data, error, onRetry }: Props) {
  if (error) {
    return (
      <div className="error-state">
        <div className="icon">⚠️</div>
        <h2>Something went wrong</h2>
        <p>{error.message}</p>
        <button onClick={onRetry} className="btn-primary">
          Try Again
        </button>
      </div>
    );
  }
  
  return <div>{/* render data */}</div>;
}
```

**After:**
```tsx
import { EmptyStateBlock } from '@/components/v1';
import { toAppError } from '@/services/error-mapping';

function DataView({ data, error, onRetry }: Props) {
  if (error) {
    const appError = toAppError(error);
    return (
      <EmptyStateBlock
        error={appError}
        actions={[
          { label: 'Try Again', onClick: onRetry, variant: 'primary' }
        ]}
      />
    );
  }
  
  return <div>{/* render data */}</div>;
}
```

### Example 4: Custom Content

**Before:**
```tsx
function GameLibrary({ games, onBrowse }: Props) {
  if (games.length === 0) {
    return (
      <div className="empty-state custom-empty">
        <div className="icon">🎮</div>
        <h2>Your game library is empty</h2>
        <p>Discover and play exciting blockchain games to start building your collection.</p>
        <button onClick={onBrowse} className="btn-primary">
          Browse Games
        </button>
      </div>
    );
  }
  
  return <div>{/* render games */}</div>;
}
```

**After:**
```tsx
import { EmptyStateBlock } from '@/components/v1';

function GameLibrary({ games, onBrowse }: Props) {
  if (games.length === 0) {
    return (
      <EmptyStateBlock
        icon="🎮"
        title="Your game library is empty"
        description="Discover and play exciting blockchain games to start building your collection."
        actions={[
          { label: 'Browse Games', onClick: onBrowse, variant: 'primary' }
        ]}
      />
    );
  }
  
  return <div>{/* render games */}</div>;
}
```

## Migration Checklist

### Step 1: Identify Empty States
- [ ] Search codebase for empty state implementations
- [ ] Look for patterns like `if (data.length === 0)` or `if (!data)`
- [ ] Identify error state handling

### Step 2: Import Component
```tsx
import { EmptyStateBlock } from '@/components/v1';
```

### Step 3: Choose Variant
Determine which variant best fits your use case:
- `list` - Empty collections, arrays, lists
- `search` - No search results
- `transaction` - Empty transaction history
- `error` - Error states
- `default` - Generic empty states

### Step 4: Add Actions (if needed)
```tsx
actions={[
  { label: 'Action Label', onClick: handleAction, variant: 'primary' }
]}
```

### Step 5: Integrate Errors (if applicable)
```tsx
import { toAppError } from '@/services/error-mapping';

const appError = toAppError(rawError);
<EmptyStateBlock error={appError} />
```

### Step 6: Remove Old Code
- [ ] Remove custom empty state components
- [ ] Remove custom CSS for empty states
- [ ] Update tests to use new component

### Step 7: Test
- [ ] Verify visual appearance
- [ ] Test action callbacks
- [ ] Test error states
- [ ] Test accessibility (keyboard navigation, screen readers)

## Common Patterns

### Pattern 1: Conditional Rendering
```tsx
function MyComponent({ data }: Props) {
  if (loading) return <LoadingSpinner />;
  if (error) return <EmptyStateBlock error={toAppError(error)} />;
  if (data.length === 0) return <EmptyStateBlock variant="list" />;
  
  return <DataView data={data} />;
}
```

### Pattern 2: With Filters
```tsx
function FilteredList({ items, filters, onClearFilters }: Props) {
  if (items.length === 0) {
    const hasFilters = Object.keys(filters).length > 0;
    
    return (
      <EmptyStateBlock
        variant="search"
        title={hasFilters ? "No results match your filters" : "No items yet"}
        description={hasFilters ? "Try removing some filters." : undefined}
        actions={hasFilters ? [
          { label: 'Clear Filters', onClick: onClearFilters, variant: 'primary' }
        ] : undefined}
      />
    );
  }
  
  return <div>{/* render items */}</div>;
}
```

### Pattern 3: Multiple Actions
```tsx
<EmptyStateBlock
  variant="search"
  actions={[
    { label: 'Clear Filters', onClick: handleClearFilters, variant: 'primary' },
    { label: 'Reset Search', onClick: handleResetSearch },
    { label: 'Browse All', onClick: handleBrowseAll }
  ]}
/>
```

### Pattern 4: Async Actions
```tsx
const [isRetrying, setIsRetrying] = useState(false);

const handleRetry = async () => {
  setIsRetrying(true);
  try {
    await fetchData();
  } finally {
    setIsRetrying(false);
  }
};

<EmptyStateBlock
  error={error}
  actions={[
    {
      label: isRetrying ? 'Retrying...' : 'Retry',
      onClick: handleRetry,
      variant: 'primary',
      disabled: isRetrying
    }
  ]}
/>
```

## Styling Customization

If you need custom styling:

```tsx
<EmptyStateBlock
  variant="list"
  className="my-custom-empty-state"
/>
```

```css
.my-custom-empty-state {
  background-color: #f9fafb;
  border-radius: 8px;
  border: 1px dashed #d1d5db;
}
```

## Breaking Changes

None - this is a new component. However, if you're replacing existing custom implementations:

1. **CSS Classes**: Old custom classes won't work. Use the `className` prop or override component CSS classes.
2. **Icons**: If you were using a specific icon library, you may need to pass custom icon components.
3. **Styling**: The component has its own styling. Custom styles may need adjustment.

## Troubleshooting

### Issue: Actions not appearing
**Solution**: Ensure actions array is not empty and each action has a valid `onClick` callback.

### Issue: Custom icon not showing
**Solution**: Pass icon as a prop: `icon={<MyIcon />}` or `icon="🎮"`

### Issue: Description not showing
**Solution**: Check if `description` is set to `null`. Use `undefined` or omit the prop to use variant default.

### Issue: Styling looks different
**Solution**: Use `className` prop to add custom styles or override component CSS classes.

## Need Help?

- Check the [README](./EmptyStateBlock.README.md) for full documentation
- View [examples](./EmptyStateBlock.examples.tsx) for usage patterns
- Run the [demo](./EmptyStateBlock.demo.tsx) to see all features
- Check [tests](../../../tests/components/v1/EmptyStateBlock.test.tsx) for edge cases

## Rollback Plan

If you need to rollback:

1. Keep old empty state components temporarily
2. Migrate incrementally (one feature at a time)
3. Test thoroughly before removing old code
4. Use feature flags if available

## Timeline Recommendation

- **Week 1**: Migrate high-traffic pages (dashboard, main lists)
- **Week 2**: Migrate search and filter pages
- **Week 3**: Migrate error states
- **Week 4**: Migrate remaining pages and remove old code

## Success Metrics

Track these metrics to measure migration success:
- Reduced code duplication
- Improved accessibility scores
- Consistent user experience
- Reduced bug reports for empty states
- Faster development of new features
