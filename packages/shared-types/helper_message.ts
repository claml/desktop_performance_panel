// packages/shared-types/helper_message.ts
// Contract version: 1.0.0
// Source schema: packages/contracts/helper_message.schema.json
//
// Envelope protocol for all stdout output from hardware-helper.exe.
// Every line is a complete HelperMessage JSON object.

import type { HardwareSnapshot } from './hardware';

export type HelperMessageType = 'init' | 'snapshot' | 'error' | 'status';

export interface HelperMessageBase {
  type: HelperMessageType;
  version: string;
  timestamp: number;
}

export interface HelperMessageInit extends HelperMessageBase {
  type: 'init';
}

export interface HelperMessageSnapshot extends HelperMessageBase {
  type: 'snapshot';
  data: HardwareSnapshot;
}

export interface HelperMessageError extends HelperMessageBase {
  type: 'error';
  message: string;
  recoverable: boolean;
}

export interface HelperMessageStatus extends HelperMessageBase {
  type: 'status';
  message: string;
}

export type HelperMessage =
  | HelperMessageInit
  | HelperMessageSnapshot
  | HelperMessageError
  | HelperMessageStatus;

/**
 * Type guard: checks if a HelperMessage is a snapshot message.
 * Useful for filtering snapshot lines from the stdout stream.
 */
export function isSnapshotMessage(msg: HelperMessage): msg is HelperMessageSnapshot {
  return msg.type === 'snapshot';
}

/**
 * Type guard: checks if an error is recoverable.
 * Recoverable errors: keep collecting. Non-recoverable: helper is about to exit.
 */
export function isRecoverableError(msg: HelperMessage): msg is HelperMessageError {
  return msg.type === 'error' && msg.recoverable === true;
}
