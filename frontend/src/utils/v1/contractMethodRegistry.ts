/**
 * Integration helper for useContractWrite with SorobanContractClient.
 * 
 * This utility provides a bridge between the generic useContractWrite hook
 * and the specific contract methods on SorobanContractClient.
 */

import type { SorobanContractClient } from '../../services/soroban-contract-client';
import type { ContractWriteOptions } from '../../types/contracts/write';

/**
 * Contract method registry mapping method names to client calls.
 * Extend this as new contract methods are added.
 */
export const CONTRACT_METHOD_REGISTRY = {
  // Prize Pool
  'pool_fund': (client: any, params: any[]) => 
    client.pool_fund(params[0], params[1], params[2]),
  'pool_reserve': (client: any, params: any[]) => 
    client.pool_reserve(params[0], params[1], params[2]),
  'pool_release': (client: any, params: any[]) => 
    client.pool_release(params[0], params[1], params[2]),
  'pool_payout': (client: any, params: any[]) => 
    client.pool_payout(params[0], params[1], params[2], params[3]),

  // Achievement Badge
  'badge_define': (client: any, params: any[]) => 
    client.badge_define(params[0], params[1], params[2]),
  'badge_award': (client: any, params: any[]) => 
    client.badge_award(params[0], params[1], params[2]),

  // Access Control
  'access_grantRole': (client: any, params: any[]) => 
    client.access_grantRole(params[0], params[1]),
  'access_revokeRole': (client: any, params: any[]) => 
    client.access_revokeRole(params[0], params[1]),

  // Coin Flip (example)
  'coinflip_placeBet': (client: any, params: any[]) => 
    client.coinflip_placeBet(params[0], params[1], params[2]),
  'coinflip_resolveBet': (client: any, params: any[]) => 
    client.coinflip_resolveBet(params[0]),
} as const;

export type ContractMethodName = keyof typeof CONTRACT_METHOD_REGISTRY;

/**
 * Execute a contract method through the client.
 * 
 * This function is used internally by useContractWrite to bridge
 * the generic write() interface to specific client methods.
 */
export async function executeContractMethod<TData>(
  client: SorobanContractClient,
  method: string,
  params: any[] = [],
  options?: ContractWriteOptions,
): Promise<{ data: TData; txHash?: string; confirmations?: number }> {
  const executor = CONTRACT_METHOD_REGISTRY[method as ContractMethodName];
  
  if (!executor) {
    throw new Error(
      `Contract method '${method}' not found in registry. ` +
      `Available methods: ${Object.keys(CONTRACT_METHOD_REGISTRY).join(', ')}`
    );
  }

  // Execute the method through the client
  const result = await executor(client, params);

  // SorobanContractClient returns ContractResult<T>
  if (!result.success) {
    throw result.error;
  }

  return {
    data: result.data as TData,
    txHash: result.txHash,
    ledger: result.ledger,
    confirmations: result.ledger ? 1 : 0,
  };
}

/**
 * Type-safe wrapper for specific contract methods.
 * Use this to create strongly-typed hooks for specific contracts.
 * 
 * @example
 * ```typescript
 * // Create a type-safe prize pool hook
 * function usePrizePoolWrite() {
 *   const base = useContractWrite<PoolState>();
 *   
 *   const reserve = (admin: string, gameId: bigint, amount: bigint) =>
 *     base.write('pool_reserve', [admin, gameId, amount]);
 *   
 *   return { ...base, reserve };
 * }
 * ```
 */
export function createTypedContractWrite<TData>(
  methodName: ContractMethodName,
) {
  return {
    methodName,
    execute: (client: SorobanContractClient, params: any[], options?: ContractWriteOptions) =>
      executeContractMethod<TData>(client, methodName, params, options),
  };
}
