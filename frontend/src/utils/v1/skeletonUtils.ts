/**
 * Utility functions for LoadingSkeletonSet to help build and parse dimension props.
 */

// Concatenates classes filtering out falsy values
export function classNames(...classes: (string | undefined | null | false)[]): string {
    return classes.filter(Boolean).join(" ");
}

// Parses string or number props into robust CSS dimension values
export function parseDimension(value?: string | number): string | undefined {
    if (value === undefined) return undefined;
    if (typeof value === "number") return `${value}px`;
    return value;
}
