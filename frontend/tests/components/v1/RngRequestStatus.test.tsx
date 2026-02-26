import { render, screen, fireEvent } from '@testing-library/react';
import '@testing-library/jest-dom';
import { vi } from 'vitest';
import { RngRequestStatusComponent } from '../../../src/components/v1/RngRequestStatus';
import { RngRequestStatus } from '../../../src/types/contracts/rng';

describe('RngRequestStatus', () => {
    const mockRequest = {
        requestId: 'RNG_123',
        status: RngRequestStatus.Pending,
        requestedAt: Date.now(),
    };

    it('renders pending state correctly', () => {
        render(<RngRequestStatusComponent request={mockRequest} />);

        expect(screen.getByText('RNG Request Status')).toBeInTheDocument();
        expect(screen.getByText('ID: RNG_123')).toBeInTheDocument();
        expect(screen.getByText('Pending')).toBeInTheDocument();
        expect(screen.getByText('Waiting for blockchain confirmation...')).toBeInTheDocument();
    });

    it('renders fulfilled state with result', () => {
        const fulfilledRequest = {
            ...mockRequest,
            status: RngRequestStatus.Fulfilled,
            result: 42,
            fulfilledAt: Date.now(),
        };

        render(<RngRequestStatusComponent request={fulfilledRequest} />);

        expect(screen.getByText('Fulfilled')).toBeInTheDocument();
        expect(screen.getByTestId('rng-request-status-result')).toHaveTextContent('42');
    });

    it('renders failed state if error is provided', () => {
        const error = new Error('Network timeout');
        render(<RngRequestStatusComponent error={error} />);

        expect(screen.getByText('Error')).toBeInTheDocument();
        expect(screen.getByText('Network timeout')).toBeInTheDocument();
    });

    it('renders empty state if no request data', () => {
        render(<RngRequestStatusComponent request={null} />);

        expect(screen.getByText('No RNG request data available.')).toBeInTheDocument();
    });

    it('handles refresh callback', () => {
        const onRefresh = vi.fn();
        render(<RngRequestStatusComponent request={mockRequest} onRefresh={onRefresh} />);

        const refreshBtn = screen.getByTestId('rng-request-status-refresh-btn');
        fireEvent.click(refreshBtn);

        expect(onRefresh).toHaveBeenCalledWith('RNG_123');
    });

    it('disables refresh button when loading', () => {
        const onRefresh = vi.fn();
        render(
            <RngRequestStatusComponent
                request={mockRequest}
                onRefresh={onRefresh}
                isLoading={true}
            />
        );

        const refreshBtn = screen.getByTestId('rng-request-status-refresh-btn');
        expect(refreshBtn).toBeDisabled();
        expect(screen.getAllByText('Refreshing...')).toHaveLength(2);
    });

    it('applies custom className and label', () => {
        render(
            <RngRequestStatusComponent
                request={mockRequest}
                className="custom-class"
                label="Custom Label"
            />
        );

        expect(screen.getByTestId('rng-request-status')).toHaveClass('custom-class');
        expect(screen.getByText('Custom Label')).toBeInTheDocument();
    });
});
