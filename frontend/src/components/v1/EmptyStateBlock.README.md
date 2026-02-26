# EmptyStateBlock Component

A reusable React component for displaying empty states across the Stellarcade application. Provides standardized UI for scenarios like empty lists, no search results, missing data, and error states.

## Features

- 🎨 **Multiple Variants**: Pre-configured for common scenarios (list, search, transaction, error)
- 🔒 **Secure**: Built-in XSS protection and input sanitization
- ♿ **Accessible**: Semantic HTML, ARIA labels, keyboard navigation
- 🎯 **Type-Safe**: Full TypeScript support with comprehensive interfaces
- 🔌 **Error Integration**: Seamless integration with AppError system
- 📱 **Responsive**: Mobile-friendly with adaptive layouts
- 🎭 **Customizable**: Override any default with custom props

## Installation

```tsx
import { EmptyStateBlock } from '@/components/v1';
// or
import { EmptyStateBlock } from '@/components/v1/EmptyStateBlock';
```

## Basic Usage

### Using Variants

```tsx
// Empty list
<EmptyStateBlock variant="list" />

// No search results
<EmptyStateBlock variant="search" />

// No transactions
<EmptyStateBlock variant="transaction" />

// Error state
<EmptyStateBlock variant="error" />

// Default/generic
<EmptyStateBlock variant="default" />
```

### Custom Content

```tsx
<EmptyStateBlock
  icon="🎮"
  title="No games available"
  description="Check back later for new games!"
/>
```

### With Actions

```tsx
<EmptyStateBlock
  variant="search"
  actions={[
    { 
      label: 'Clear Filters', 
      onClick: handleClearFilters, 
      variant: 'primary' 
    },
    { 
      label: 'Go Back', 
      onClick: handleGoBack 
    }
  ]}
/>
```

### Error Integration

```tsx
import { toAppError } from '@/services/error-mapping';

try {
  await fetchData();
} catch (error) {
  const appError = toAppError(error);
  
  return (
    <EmptyStateBlock
      error={appError}
      actions={[
        { 
          label: 'Retry', 
          onClick: handleRetry, 
          variant: 'primary' 
        }
      ]}
    />
  );
}
```

## API Reference

### Props

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `variant` | `'list' \| 'search' \| 'transaction' \| 'error' \| 'default'` | `'default'` | Pre-configured context variant |
| `icon` | `React.ReactNode \| string \| null` | Variant default | Custom icon (emoji, SVG, component, or null to hide) |
| `title` | `string` | Variant default | Title text |
| `description` | `string \| null` | Variant default | Description text (null to hide) |
| `actions` | `EmptyStateAction[]` | `undefined` | Array of action buttons |
| `error` | `AppError` | `undefined` | AppError object for error states |
| `className` | `string` | `undefined` | Custom CSS class |
| `testId` | `string` | `'empty-state-block'` | Test ID for testing |

### EmptyStateAction Interface

```typescript
interface EmptyStateAction {
  label: string;                           // Button text
  onClick: () => void | Promise<void>;     // Click handler
  variant?: 'primary' | 'secondary';       // Visual style (default: 'secondary')
  disabled?: boolean;                      // Disabled state (default: false)
}
```

## Variants

### List Variant
```tsx
<EmptyStateBlock variant="list" />
```
- **Icon**: 📋
- **Title**: "No items yet"
- **Description**: "Items will appear here once they are added."

### Search Variant
```tsx
<EmptyStateBlock variant="search" />
```
- **Icon**: 🔍
- **Title**: "No results found"
- **Description**: "Try adjusting your search terms or filters."

### Transaction Variant
```tsx
<EmptyStateBlock variant="transaction" />
```
- **Icon**: 🧾
- **Title**: "No transactions"
- **Description**: "Your transaction history will appear here."

### Error Variant
```tsx
<EmptyStateBlock variant="error" />
```
- **Icon**: ⚠️
- **Title**: "Something went wrong"
- **Description**: "An error occurred while loading this content."

### Default Variant
```tsx
<EmptyStateBlock variant="default" />
```
- **Icon**: ℹ️
- **Title**: "No data available"
- **Description**: "There is currently no data to display."

## Advanced Examples

### Override Variant Defaults

```tsx
<EmptyStateBlock
  variant="list"
  title="Your game library is empty"
  description="Start playing games to build your collection!"
  icon="🎯"
/>
```

### Multiple Actions

```tsx
<EmptyStateBlock
  variant="search"
  actions={[
    { 
      label: 'Clear All Filters', 
      onClick: handleClearAll, 
      variant: 'primary' 
    },
    { 
      label: 'Reset Search', 
      onClick: handleResetSearch 
    },
    { 
      label: 'Browse All', 
      onClick: handleBrowseAll 
    }
  ]}
/>
```

### Custom Icon Component

```tsx
import { AlertCircle } from 'lucide-react';

<EmptyStateBlock
  icon={<AlertCircle size={48} />}
  title="Connection Lost"
  description="Unable to connect to the server."
  actions={[
    { label: 'Retry Connection', onClick: handleRetry, variant: 'primary' }
  ]}
/>
```

### Conditional Description

```tsx
<EmptyStateBlock
  variant="list"
  description={hasFilters ? "No items match your filters." : null}
/>
```

### Error with Retry Logic

```tsx
const [error, setError] = useState<AppError | null>(null);

const handleRetry = async () => {
  setError(null);
  try {
    await fetchData();
  } catch (err) {
    setError(toAppError(err));
  }
};

if (error) {
  return (
    <EmptyStateBlock
      error={error}
      actions={[
        { 
          label: 'Try Again', 
          onClick: handleRetry, 
          variant: 'primary' 
        }
      ]}
    />
  );
}
```

## Styling

### Custom Styling

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

### CSS Variables (Optional Enhancement)

The component uses standard CSS classes that can be customized:

- `.empty-state-block` - Container
- `.empty-state-block__icon` - Icon wrapper
- `.empty-state-block__title` - Title text
- `.empty-state-block__description` - Description text
- `.empty-state-block__actions` - Actions container
- `.empty-state-block__action-button` - Action button
- `.empty-state-block__action-button--primary` - Primary button
- `.empty-state-block__action-button--secondary` - Secondary button

## Accessibility

The component follows accessibility best practices:

- ✅ Semantic HTML (`<section>`, `<h2>`, `<p>`, `<button>`)
- ✅ ARIA attributes (`role="status"`, `aria-live="polite"`, `aria-hidden="true"`)
- ✅ Keyboard navigation (all buttons are keyboard accessible)
- ✅ Focus indicators (visible focus outlines)
- ✅ Screen reader friendly (proper heading hierarchy)

## Security

The component includes built-in security features:

- ✅ XSS protection via input sanitization
- ✅ Script tag removal from all string props
- ✅ Event handler sanitization
- ✅ Safe callback wrappers (errors don't crash the UI)
- ✅ TypeScript type safety

## Testing

```tsx
import { render, screen, fireEvent } from '@testing-library/react';
import { EmptyStateBlock } from '@/components/v1';

test('renders with variant', () => {
  render(<EmptyStateBlock variant="list" />);
  expect(screen.getByText('No items yet')).toBeInTheDocument();
});

test('calls action callback on click', () => {
  const handleClick = jest.fn();
  render(
    <EmptyStateBlock
      variant="list"
      actions={[{ label: 'Add Item', onClick: handleClick }]}
    />
  );
  
  fireEvent.click(screen.getByText('Add Item'));
  expect(handleClick).toHaveBeenCalledTimes(1);
});
```

## Best Practices

### ✅ Do

- Use appropriate variants for context
- Provide clear, actionable titles and descriptions
- Include retry actions for error states
- Override defaults when needed for better UX
- Use primary variant for the main action

### ❌ Don't

- Don't use for loading states (use a loading spinner instead)
- Don't include too many actions (2-3 max recommended)
- Don't use overly technical error messages
- Don't forget to handle action callbacks properly
- Don't use HTML in string props (it will be sanitized)

## Browser Support

- ✅ Chrome/Edge (latest)
- ✅ Firefox (latest)
- ✅ Safari (latest)
- ✅ Mobile browsers (iOS Safari, Chrome Mobile)

## Performance

- Stateless component (no internal state)
- Minimal re-renders
- Small bundle size (~3KB gzipped)
- No heavy dependencies
- CSS-only animations

## Related Components

- `LoadingSpinner` - For loading states
- `ErrorBoundary` - For catching React errors
- `Toast` - For temporary notifications

## Changelog

### v1.0.0 (Initial Release)
- Initial implementation with all core features
- Support for 5 variants
- Error integration
- Full accessibility support
- Comprehensive documentation

## License

Part of the Stellarcade project.
