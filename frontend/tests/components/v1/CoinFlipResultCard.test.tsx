import React from 'react';
import { render, screen, fireEvent } from '@testing-library/react';
import { CoinFlipResultCard } from '../../../src/components/v1/CoinFlipResultCard';
import { CoinFlipGame, CoinFlipGameState, CoinFlipSide } from '../../../src/types/contracts/coinFlip';

describe('CoinFlipResultCard', () => {
    const mockGame: CoinFlipGame = {
        id: 'game-123',
        wager: '10000000', // 1 XLM -> 10_000_000 stroops
        side: CoinFlipSide.Heads,
        status: CoinFlipGameState.Placed
    };

    it('renders loading state', () => {
        render(<CoinFlipResultCard isLoading />);
        expect(screen.getByTestId('coinflip-skeleton')).toBeTruthy();
    });

    it('renders error state', () => {
        const error = new Error('Network failed');
        render(<CoinFlipResultCard error={error} />);
        expect(screen.getByTestId('coinflip-error')).toBeTruthy();
        expect(screen.getByText(/Network failed/i)).toBeTruthy();
    });

    it('renders retry button and triggers callback', () => {
        const handleRetry = jest.fn();
        const error = new Error('Retry me');
        render(<CoinFlipResultCard error={error} onRetry={handleRetry} />);

        const retryBtn = screen.getByRole('button', { name: /retry/i });
        fireEvent.click(retryBtn);
        expect(handleRetry).toHaveBeenCalledTimes(1);
    });

    it('renders empty state', () => {
        render(<CoinFlipResultCard game={null} />);
        expect(screen.getByTestId('coinflip-empty')).toBeTruthy();
    });

    it('renders pending game', () => {
        render(<CoinFlipResultCard game={mockGame} />);
        expect(screen.getByTestId('coinflip-content')).toBeTruthy();
        expect(screen.getByText('game-123')).toBeTruthy();
        expect(screen.getByText('1 XLM')).toBeTruthy();
        expect(screen.getByText('Placed')).toBeTruthy();
    });

    it('renders resolve button for pending game when onResolve is provided', () => {
        const handleResolve = jest.fn();
        render(<CoinFlipResultCard game={mockGame} onResolve={handleResolve} />);

        const resolveBtn = screen.getByRole('button', { name: /resolve game/i });
        fireEvent.click(resolveBtn);
        expect(handleResolve).toHaveBeenCalledWith('game-123');
    });

    it('renders win state correctly', () => {
        const resolvedGame: CoinFlipGame = {
            ...mockGame,
            status: CoinFlipGameState.Resolved,
            winner: 'player-xyz'
        };
        render(<CoinFlipResultCard game={resolvedGame} currentWalletAddress="player-xyz" />);

        expect(screen.getByText('Resolved')).toBeTruthy();
        expect(screen.getByText(/You Won!/i)).toBeTruthy();
        expect(screen.getByText('2 XLM')).toBeTruthy(); // Payout is double
    });

    it('renders loss state correctly', () => {
        const resolvedGame: CoinFlipGame = {
            ...mockGame,
            status: CoinFlipGameState.Resolved,
            winner: 'house-xyz'
        };
        render(<CoinFlipResultCard game={resolvedGame} currentWalletAddress="player-xyz" />);

        expect(screen.getByText('Resolved')).toBeTruthy();
        expect(screen.getByText(/Better luck next time/i)).toBeTruthy();
        expect(screen.getByText('0 XLM')).toBeTruthy();
    });
});
