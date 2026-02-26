import React from 'react';
import './LoadingSkeletonSet.css';
import { classNames, parseDimension } from '../../utils/v1/skeletonUtils';

export interface SkeletonBaseProps extends React.HTMLAttributes<HTMLDivElement> {
    width?: string | number;
    height?: string | number;
    borderRadius?: string | number;
    className?: string;
    circle?: boolean;
}

export function SkeletonBase({ width, height, borderRadius, className, circle, style, ...rest }: SkeletonBaseProps) {
    const dynamicRadius = circle ? '50%' : parseDimension(borderRadius);
    return (
        <div
            className={classNames('stellarcade-skeleton', 'stellarcade-skeleton-base', className)}
            style={{
                width: parseDimension(width),
                height: parseDimension(height) || '1rem',
                borderRadius: dynamicRadius,
                ...style,
            }}
            data-testid="skeleton-base"
            {...rest}
        />
    );
}

export interface SkeletonCardProps extends React.HTMLAttributes<HTMLDivElement> {
    className?: string;
    children?: React.ReactNode;
}

export function SkeletonCard({ className, children, ...rest }: SkeletonCardProps) {
    return (
        <div
            className={classNames('stellarcade-skeleton-card', className)}
            data-testid="skeleton-card"
            {...rest}
        >
            {children ? children : (
                <>
                    <SkeletonBase height="150px" borderRadius="0.5rem" />
                    <SkeletonBase height="24px" width="75%" />
                    <SkeletonBase height="16px" width="50%" />
                </>
            )}
        </div>
    );
}

export interface SkeletonRowProps extends React.HTMLAttributes<HTMLDivElement> {
    className?: string;
    avatarSize?: string | number;
}

export function SkeletonRow({ className, avatarSize = "40px", ...rest }: SkeletonRowProps) {
    return (
        <div
            className={classNames('stellarcade-skeleton-row', className)}
            data-testid="skeleton-row"
            {...rest}
        >
            <SkeletonBase width={avatarSize} height={avatarSize} circle />
            <div style={{ flex: 1, display: 'flex', flexDirection: 'column', gap: '0.5rem' }}>
                <SkeletonBase height="16px" width="60%" />
                <SkeletonBase height="12px" width="40%" />
            </div>
        </div>
    );
}

export interface SkeletonListProps extends React.HTMLAttributes<HTMLDivElement> {
    className?: string;
    count?: number;
    type?: 'row' | 'card';
}

export function SkeletonList({ className, count = 3, type = 'row', ...rest }: SkeletonListProps) {
    return (
        <div
            className={classNames('stellarcade-skeleton-list', className)}
            data-testid="skeleton-list"
            {...rest}
        >
            {Array.from({ length: Math.max(0, count) }).map((_, i) => (
                type === 'row' ? <SkeletonRow key={`skeleton-row-${i}`} /> : <SkeletonCard key={`skeleton-card-${i}`} />
            ))}
        </div>
    );
}

export interface LoadingStateProps {
    isLoading: boolean;
    error?: Error | null;
    empty?: boolean;
    children: React.ReactNode;
    fallback?: React.ReactNode;
    errorFallback?: (error: Error) => React.ReactNode;
    emptyFallback?: React.ReactNode;
}

export function LoadingState({
    isLoading,
    error,
    empty,
    children,
    fallback = <SkeletonList count={3} />,
    errorFallback,
    emptyFallback
}: LoadingStateProps) {
    if (error) {
        if (errorFallback) return <>{errorFallback(error)}</>;
        return (
            <div className="stellarcade-error-state" data-testid="skeleton-error">
                Failed to load data: {error.message}
            </div>
        );
    }

    if (isLoading) {
        return <>{fallback}</>;
    }

    if (empty) {
        if (emptyFallback) return <>{emptyFallback}</>;
        return (
            <div className="stellarcade-empty-state" data-testid="skeleton-empty">
                No data available
            </div>
        );
    }

    return <>{children}</>;
}
