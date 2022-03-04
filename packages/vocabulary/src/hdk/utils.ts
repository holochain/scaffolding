import {
  AppInfoResponse,
  CellId,
  EntryHash,
  AgentPubKey,
  HeaderHash,
} from "@holochain/client";
import { Base64 } from "js-base64";

export function deserializeHash(hash: string): Uint8Array {
  return Base64.toUint8Array(hash.slice(1));
}

export function serializeHash(hash: Uint8Array): string {
  return `u${Base64.fromUint8Array(hash, true)}`;
}

export function getCellIdForDnaHash(
  appInfo: AppInfoResponse,
  dnaHash: string
): CellId {
  const cell = appInfo.cell_data.find(
    (cellData) => serializeHash(cellData.cell_id[0]) === dnaHash
  );

  if (!cell) throw new Error(`Could not find cell for dna ${dnaHash}`);

  return cell.cell_id;
}

/** From https://github.com/holochain/holochain/blob/develop/crates/holo_hash/src/hash_type/primitive.rs */

export function fakeEntryHash(): EntryHash {
  return new Uint8Array([0x84, 0x21, 0x24, ...randomByteArray(36)]);
}

export function fakeAgentPubKey(): AgentPubKey {
  return new Uint8Array([0x84, 0x20, 0x24, ...randomByteArray(36)]);
}

export function fakeHeaderHash(): HeaderHash {
  return new Uint8Array([0x84, 0x29, 0x24, ...randomByteArray(36)]);
}

function randomByteArray(n: number): Uint8Array {
  const QUOTA = 65536;
  const a = new Uint8Array(n);
  for (let i = 0; i < n; i += QUOTA) {
    crypto.getRandomValues(a.subarray(i, i + Math.min(n - i, QUOTA)));
  }
  return a;
}
