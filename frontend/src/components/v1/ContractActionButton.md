# ContractActionButton

Guarded mutation trigger for contract actions. Handles preconditions, in-flight locking, and mapped errors.

## Example

```tsx
<ContractActionButton
  label="Flip"
  action={() => submitFlipTx()}
  walletConnected={isWalletConnected}
  networkSupported={isSupportedNetwork}
  onSuccess={(result) => onTxSubmitted(result)}
  onError={(err) => trackError(err)}
/>
```
