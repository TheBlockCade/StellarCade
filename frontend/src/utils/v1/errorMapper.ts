/**
 * Error Notice Utilities - v1
 *
 * User-friendly error message mapping and presentation utilities
 * for the ErrorNotice component. Sanitizes and normalizes domain errors
 * into safe, user-facing content.
 */

import { AppError, ErrorDomain, ErrorSeverity } from "../../types/errors";

// ---------------------------------------------------------------------------
// User-facing message mapping
// ---------------------------------------------------------------------------

/**
 * Mapping of error codes to user-friendly messages.
 * These messages are safe to display directly to users.
 */
const USER_FRIENDLY_MESSAGES: Record<string, string> = {
  // RPC Errors
  RPC_NODE_UNAVAILABLE:
    "The network is temporarily unavailable. Please try again.",
  RPC_CONNECTION_TIMEOUT:
    "Request timed out. Please check your connection and try again.",
  RPC_SIMULATION_FAILED: "Transaction simulation failed. Please try again.",
  RPC_TX_REJECTED: "Transaction was rejected by the network.",
  RPC_TX_EXPIRED: "Transaction expired. Please try again.",
  RPC_RESOURCE_LIMIT_EXCEEDED:
    "Transaction is too complex. Please simplify and try again.",
  RPC_INVALID_RESPONSE: "Invalid response from network. Please try again.",
  RPC_UNKNOWN: "Network error occurred. Please try again.",

  // API Errors
  API_NETWORK_ERROR:
    "Cannot connect to Stellarcade servers. Please check your connection.",
  API_UNAUTHORIZED: "Please sign in to continue.",
  API_FORBIDDEN: "You don't have permission for this action.",
  API_NOT_FOUND: "The requested resource was not found.",
  API_VALIDATION_ERROR: "Please check your input and try again.",
  API_RATE_LIMITED: "Too many requests. Please wait a moment and try again.",
  API_SERVER_ERROR: "Server error occurred. Please try again shortly.",
  API_UNKNOWN: "An error occurred. Please try again.",

  // Wallet Errors
  WALLET_NOT_INSTALLED:
    "Freighter wallet is required. Please install it to continue.",
  WALLET_NOT_CONNECTED: "Please connect your wallet to continue.",
  WALLET_USER_REJECTED: "Transaction was cancelled.",
  WALLET_NETWORK_MISMATCH: "Please switch to the correct network.",
  WALLET_INSUFFICIENT_BALANCE: "Insufficient balance for this transaction.",
  WALLET_SIGN_FAILED: "Failed to sign transaction. Please try again.",
  WALLET_UNKNOWN: "Wallet error occurred. Please try again.",

  // Contract Errors
  CONTRACT_ALREADY_INITIALIZED: "Contract is already set up.",
  CONTRACT_NOT_INITIALIZED: "Contract is not initialized.",
  CONTRACT_NOT_AUTHORIZED: "You're not authorized to perform this action.",
  CONTRACT_INVALID_AMOUNT: "Invalid amount. Please enter a valid value.",
  CONTRACT_INSUFFICIENT_FUNDS: "Insufficient funds available.",
  CONTRACT_GAME_ALREADY_RESERVED: "Game is already in progress.",
  CONTRACT_RESERVATION_NOT_FOUND: "No active game found.",
  CONTRACT_PAYOUT_EXCEEDS_RESERVATION: "Payout exceeds available funds.",
  CONTRACT_OVERFLOW: "Number too large. Please use a smaller value.",
  CONTRACT_INVALID_BOUND: "Invalid parameters. Please check your input.",
  CONTRACT_DUPLICATE_REQUEST_ID: "Duplicate request. Please try again.",
  CONTRACT_REQUEST_NOT_FOUND: "Request not found or expired.",
  CONTRACT_ALREADY_FULFILLED: "This request has already been processed.",
  CONTRACT_UNAUTHORIZED_CALLER: "Unauthorized action.",
  CONTRACT_UNKNOWN: "Contract error occurred. Please try again.",

  // Fallback
  UNKNOWN: "An unexpected error occurred. Please try again.",
};

/**
 * Severity-based action suggestions for users.
 */
const SEVERITY_ACTIONS: Record<ErrorSeverity, string | null> = {
  [ErrorSeverity.RETRYABLE]: "You can try again.",
  [ErrorSeverity.USER_ACTIONABLE]: null, // Message should be specific enough
  [ErrorSeverity.FATAL]: "Please contact support if this continues.",
};

// ---------------------------------------------------------------------------
// Public interfaces
// ---------------------------------------------------------------------------

export interface ErrorNoticeData {
  /** User-friendly error message */
  message: string;
  /** Suggested action based on severity */
  action?: string;
  /** Error severity for visual styling */
  severity: ErrorSeverity;
  /** Error domain for categorization */
  domain: ErrorDomain;
  /** Machine-readable error code */
  code: string;
  /** Whether retry is suggested */
  canRetry: boolean;
  /** Whether error is user-actionable */
  isUserActionable: boolean;
  /** Debug information (only in development) */
  debug?: {
    originalError?: unknown;
    context?: Record<string, unknown>;
    retryAfterMs?: number;
  };
}

export interface ErrorNoticeOptions {
  /** Include debug information in output */
  includeDebug?: boolean;
  /** Override default user-friendly message */
  customMessage?: string;
  /** Override default action suggestion */
  customAction?: string;
}

// ---------------------------------------------------------------------------
// Core utilities
// ---------------------------------------------------------------------------

/**
 * Convert an AppError into user-friendly ErrorNoticeData.
 *
 * Sanitizes the error for safe user display and adds presentation hints.
 */
export function normalizeErrorForDisplay(
  error: AppError,
  options: ErrorNoticeOptions = {},
): ErrorNoticeData {
  const {
    includeDebug = false, // Default to false for production safety
    customMessage,
    customAction,
  } = options;

  // Get user-friendly message or fallback
  const message =
    customMessage || USER_FRIENDLY_MESSAGES[error.code] || error.message;

  // Get action suggestion
  const action = customAction || SEVERITY_ACTIONS[error.severity] || undefined;

  // Determine retry capability
  const canRetry = error.severity === ErrorSeverity.RETRYABLE;
  const isUserActionable = error.severity === ErrorSeverity.USER_ACTIONABLE;

  const baseData: ErrorNoticeData = {
    message,
    action,
    severity: error.severity,
    domain: error.domain,
    code: error.code,
    canRetry,
    isUserActionable,
  };

  // Add debug info in development or when explicitly requested
  if (includeDebug) {
    baseData.debug = {
      originalError: error.originalError,
      context: error.context,
      retryAfterMs: error.retryAfterMs,
    };
  }

  return baseData;
}

/**
 * Get CSS class names for error severity styling.
 */
export function getErrorSeverityClasses(severity: ErrorSeverity): string {
  const baseClasses = "error-notice";

  switch (severity) {
    case ErrorSeverity.RETRYABLE:
      return `${baseClasses} error-notice--retryable`;
    case ErrorSeverity.USER_ACTIONABLE:
      return `${baseClasses} error-notice--user-actionable`;
    case ErrorSeverity.FATAL:
      return `${baseClasses} error-notice--fatal`;
    default:
      return baseClasses;
  }
}

/**
 * Get icon name for error severity.
 */
export function getErrorSeverityIcon(severity: ErrorSeverity): string {
  switch (severity) {
    case ErrorSeverity.RETRYABLE:
      return "refresh";
    case ErrorSeverity.USER_ACTIONABLE:
      return "alert-triangle";
    case ErrorSeverity.FATAL:
      return "x-circle";
    default:
      return "alert-circle";
  }
}

/**
 * Check if an error should be automatically dismissed.
 */
export function shouldAutoDismiss(error: AppError): boolean {
  // Auto-dismiss retryable network errors after a short delay
  return (
    error.severity === ErrorSeverity.RETRYABLE &&
    error.domain === ErrorDomain.RPC &&
    error.code === "RPC_CONNECTION_TIMEOUT"
  );
}

/**
 * Get suggested auto-dismiss delay in milliseconds.
 */
export function getAutoDismissDelay(error: AppError): number {
  if (!shouldAutoDismiss(error)) {
    return 0; // No auto-dismiss
  }

  // Shorter delay for connection timeouts
  return error.code === "RPC_CONNECTION_TIMEOUT" ? 3000 : 5000;
}

/**
 * Validate ErrorNoticeData for consistency.
 */
export function validateErrorNoticeData(data: ErrorNoticeData): boolean {
  return !!(
    data.message &&
    typeof data.message === "string" &&
    data.severity &&
    data.domain &&
    data.code &&
    typeof data.canRetry === "boolean" &&
    typeof data.isUserActionable === "boolean"
  );
}

/**
 * Create a safe fallback ErrorNoticeData for unexpected errors.
 */
export function createFallbackErrorNotice(error: unknown): ErrorNoticeData {
  return {
    message: USER_FRIENDLY_MESSAGES.UNKNOWN,
    severity: ErrorSeverity.FATAL,
    domain: ErrorDomain.UNKNOWN,
    code: "UNKNOWN",
    canRetry: false,
    isUserActionable: false,
    debug: false
      ? {
          originalError: error,
        }
      : undefined,
  };
}
