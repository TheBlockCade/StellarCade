/**
 * Components v1 - Public API
 * 
 * Re-exports all v1 components for clean imports.
 * 
 * @example
 * ```tsx
 * import { EmptyStateBlock } from '@/components/v1';
 * ```
 */

export { EmptyStateBlock, default as EmptyStateBlockDefault } from './EmptyStateBlock';
export type {
  EmptyStateBlockProps,
  EmptyStateAction,
  EmptyStateVariant,
  ActionVariant,
} from './EmptyStateBlock.types';
