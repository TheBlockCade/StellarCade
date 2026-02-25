/**
 * Tests for useContractWrite hook.
 */

import { renderHook, act, waitFor } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach, type Mock } from 'vitest';
import { useContractWrite } from '../../../src/hooks/v1/useContractWrite';
import * as useSorobanClientModule from '../../../src/hooks/useSorobanClient';
import * as useWalletSessionModule from '../../../src/hooks/useWalletSession';
import * as useNetworkGuardModule from '../../../src/hooks/v1/useNetworkGuard';
import * as contractMethodRegistryModule from '../../../src/utils/v1/contractMethodRegistry';
import { ErrorDomain, ErrorSeverity } from '../../../src/types/errors';

// ── Mocks ──────────────────────────────────────────────────────────────────────

vi.mock('../../../src/hooks/useSorobanClient');
vi.mock('../../../src/hooks/useWalletSession');
vi.mock('../../../src/hooks/v1/useNetworkGuard');
vi.mock('../../../src/utils/v1/contractMethodRegistry', () => ({
  executeContractMethod: vi.fn(),
}));
vi.mock('../../../src/services/error-mapping', () => ({
  mapRpcError: vi.fn((err: any) => ({
    code: 'RPC_ERROR',
    domain: ErrorDomain.RPC,
    severity: ErrorSeverity.RETRYABLE,
    message: err.message || 'RPC error',
    originalError: err,
  })),
}));

const mockClient = {
  pool_reserve: vi.fn(),
  badge_award: vi.fn(),
};

const mockWalletSession = {
  state: 'connected',
  meta: {
    publicKey: 'GABCDEFGHJKLMNPQRSTUVWXYZ234567ABCDEFGHJKLMNPQRSTUVWXYZ2',
    network: 'testnet',
  },
};

const mockNetworkGuard = {
  network: 'testnet',
  normalizedNetwork: 'testnet',
  supportedNetworks: ['testnet', 'mainnet'],
  support: { isSupported: true, normalizedActual: 'testnet', supportedNetworks: ['testnet', 'mainnet'] },
  assertSupportedNetwork: vi.fn(() => ({
    isSupported: true,
    actual: 'testnet',
    supportedNetworks: ['testnet', 'mainnet'],
    message: '',
  })),
  isSupported: vi.fn(() => true),
};

// ── Tests ──────────────────────────────────────────────────────────────────────

describe('useContractWrite', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    (useSorobanClientModule.useSorobanClient as Mock).mockReturnValue(mockClient);
    (useWalletSessionModule.useWalletSession as Mock).mockReturnValue(mockWalletSession);
    (useNetworkGuardModule.useNetworkGuard as Mock).mockReturnValue(mockNetworkGuard);
    
    // Default: Make executeContractMethod throw an error (most tests expect failure)
    (contractMethodRegistryModule.executeContractMethod as Mock).mockRejectedValue(
      new Error('Contract method not found in registry')
    );
  });

  describe('initial state', () => {
    it('should start in idle phase', () => {
      const { result } = renderHook(() => useContractWrite());

      expect(result.current.phase).toBe('idle');
      expect(result.current.isLoading).toBe(false);
      expect(result.current.isSuccess).toBe(false);
      expect(result.current.isError).toBe(false);
      expect(result.current.confirmations).toBe(0);
      expect(result.current.txHash).toBeUndefined();
      expect(result.current.data).toBeUndefined();
      expect(result.current.error).toBeUndefined();
    });
  });

  describe('precondition validation', () => {
    it('should reject write when wallet not connected', async () => {
      // Override mock to return disconnected wallet
      (useWalletSessionModule.useWalletSession as Mock).mockReturnValue({
        state: 'disconnected',
        meta: null,
      });

      const { result } = renderHook(() => useContractWrite());

      // Call write - should reject with WALLET_NOT_CONNECTED error
      await act(async () => {
        try {
          await result.current.write('test_method');
          expect.fail('Should have thrown an error');
        } catch (error: any) {
          expect(error.code).toBe('WALLET_NOT_CONNECTED');
        }
      });

      // State should be error
      expect(result.current.phase).toBe('error');
      expect(result.current.isError).toBe(true);
    });

    it('should reject write on unsupported network', async () => {
      // Override mock for network guard
      (useNetworkGuardModule.useNetworkGuard as Mock).mockReturnValue({
        ...mockNetworkGuard,
        assertSupportedNetwork: vi.fn(() => ({
          isSupported: false,
          actual: 'unknown',
          supportedNetworks: ['testnet', 'mainnet'],
          message: 'Network unknown is not supported',
        })),
      });

      const { result } = renderHook(() => useContractWrite());

      await act(async () => {
        try {
          await result.current.write('test_method');
          expect.fail('Should have thrown an error');
        } catch (error: any) {
          expect(error.code).toBe('WALLET_NETWORK_MISMATCH');
        }
      });

      expect(result.current.phase).toBe('error');
    });

    it('should reject write with invalid method name', async () => {
      const { result } = renderHook(() => useContractWrite());

      await act(async () => {
        try {
          await result.current.write('');
          expect.fail('Should have thrown an error');
        } catch (error: any) {
          expect(error.code).toBe('API_VALIDATION_ERROR');
        }
      });

      expect(result.current.phase).toBe('error');
    });
  });

  describe('idempotency', () => {
    it('should warn on duplicate idempotency key', async () => {
      const consoleWarnSpy = vi.spyOn(console, 'warn').mockImplementation(() => {});
      const { result } = renderHook(() => useContractWrite());

      const idempotencyKey = 'test-key-123';

      // First call - should not warn
      await act(async () => {
        try {
          await result.current.write('test_method', [], { idempotencyKey });
        } catch {
          // Expected to fail
        }
      });

      // Second call with same key - should warn
      await act(async () => {
        try {
          await result.current.write('test_method', [], { idempotencyKey });
        } catch {
          // Expected to fail
        }
      });

      expect(consoleWarnSpy).toHaveBeenCalledWith(
        expect.stringContaining('Duplicate idempotency key detected'),
      );

      consoleWarnSpy.mockRestore();
    });
  });

  describe('state transitions', () => {
    it('should transition to loading then success phase', async () => {
      // Mock successful execution
      (contractMethodRegistryModule.executeContractMethod as Mock).mockResolvedValueOnce({
        data: { result: 'success' },
        txHash: 'mock-tx-hash',
        confirmations: 1,
      });

      const { result } = renderHook(() => useContractWrite());

      await act(async () => {
        await result.current.write('test_method');
      });

      // Should end in success state
      expect(result.current.phase).toBe('success');
      expect(result.current.isSuccess).toBe(true);
      expect(result.current.isLoading).toBe(false);
      expect(result.current.txHash).toBe('mock-tx-hash');
      expect(result.current.data).toEqual({ result: 'success' });
    });

    it('should transition to error phase on failure', async () => {
      const { result } = renderHook(() => useContractWrite());

      await act(async () => {
        try {
          await result.current.write('test_method');
        } catch {
          // Expected to fail
        }
      });

      await waitFor(() => {
        expect(result.current.phase).toBe('error');
        expect(result.current.isError).toBe(true);
        expect(result.current.isLoading).toBe(false);
        expect(result.current.error).toBeDefined();
      });
    });
  });

  describe('reset', () => {
    it('should reset state to idle', async () => {
      const { result } = renderHook(() => useContractWrite());

      // Trigger error state
      await act(async () => {
        try {
          await result.current.write('test_method');
        } catch {
          // Expected to fail
        }
      });
      
      expect(result.current.phase).toBe('error');

      // Reset
      act(() => {
        result.current.reset();
      });

      expect(result.current.phase).toBe('idle');
      expect(result.current.isLoading).toBe(false);
      expect(result.current.isSuccess).toBe(false);
      expect(result.current.isError).toBe(false);
      expect(result.current.error).toBeUndefined();
      expect(result.current.data).toBeUndefined();
      expect(result.current.txHash).toBeUndefined();
    });
  });

  describe('parameter validation', () => {
    it('should accept valid method and params', async () => {
      // Mock successful execution
      (contractMethodRegistryModule.executeContractMethod as Mock).mockResolvedValueOnce({
        data: { gameId: 123n },
        txHash: 'bet-tx-hash',
        confirmations: 1,
      });

      const { result } = renderHook(() => useContractWrite());

      let returnedData: any;
      await act(async () => {
        returnedData = await result.current.write('place_bet', [100_000_000n, 0], {
          idempotencyKey: 'bet-123',
          confirmationTimeoutMs: 30000,
        });
      });

      expect(result.current.isSuccess).toBe(true);
      expect(returnedData).toEqual({ gameId: 123n });
    });

    it('should handle undefined params', async () => {
      // Mock successful execution
      (contractMethodRegistryModule.executeContractMethod as Mock).mockResolvedValueOnce({
        data: { state: 'active' },
        txHash: 'state-tx-hash',
        confirmations: 1,
      });

      const { result } = renderHook(() => useContractWrite());

      let returnedData: any;
      await act(async () => {
        returnedData = await result.current.write('get_state');
      });

      expect(result.current.isSuccess).toBe(true);
      expect(returnedData).toEqual({ state: 'active' });
    });
  });

  describe('error mapping', () => {
    it('should map RPC errors through error mapper', async () => {
      // Mock executeContractMethod to throw an error
      (contractMethodRegistryModule.executeContractMethod as Mock).mockRejectedValueOnce(
        new Error('RPC connection failed')
      );

      const { result } = renderHook(() => useContractWrite());

      await act(async () => {
        try {
          await result.current.write('test_method');
          expect.fail('Should have thrown an error');
        } catch (error: any) {
          expect(error.code).toBe('RPC_ERROR');
          expect(error.domain).toBe(ErrorDomain.RPC);
        }
      });
    });
  });
});
