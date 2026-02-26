# AsyncStateBoundary

Shared renderer for async lifecycle branches (`idle`, `loading`, `success`, `error`).

## Example

```tsx
<AsyncStateBoundary
  status={status}
  data={data}
  error={error}
  onRetry={reload}
  renderLoading={() => <p>Loading...</p>}
  renderSuccess={(items) => <ItemList items={items} />}
/>
```
