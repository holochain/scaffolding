import { AppInfoResponse, CellId, EntryHash, AgentPubKey, ActionHash } from '@holochain/client';
import { Base64 } from 'js-base64';
import blake from 'blakejs';

export function deserializeHash(hash: string): Uint8Array {
  return Base64.toUint8Array(hash.slice(1));
}

export function serializeHash(hash: Uint8Array): string {
  return `u${Base64.fromUint8Array(hash, true)}`;
}

export function getCellIdForDnaHash(appInfo: AppInfoResponse, dnaHash: string): CellId {
  const cell = appInfo.cell_data.find(cellData => serializeHash(cellData.cell_id[0]) === dnaHash);

  if (!cell) throw new Error(`Could not find cell for dna ${dnaHash}`);

  return cell.cell_id;
}

/** From https://github.com/holochain/holochain/blob/develop/crates/holo_hash/src/hash_type/primitive.rs */

export function fakeEntryHash(): EntryHash {
  return new Uint8Array([0x84, 0x21, 0x24, ...fakeHoloHash()]);
}

export function fakeAgentPubKey(): AgentPubKey {
  return new Uint8Array([0x84, 0x20, 0x24, ...fakeHoloHash()]);
}

export function fakeActionHash(): ActionHash {
  return new Uint8Array([0x84, 0x29, 0x24, ...fakeHoloHash()]);
}

function fakeHoloHash(): Uint8Array {
  const hash = randomByteArray(32);
  const location = locationBytes(hash);
  return new Uint8Array([...hash, ...location]);
}

function randomByteArray(n: number): Uint8Array {
  const a = new Uint8Array(n);
  for (let i = 0; i < n; i++) {
    a[i] = Math.floor(Math.random() * 256);
  }
  return a;
}

function locationBytes(bytesHash: Uint8Array): Uint8Array {
  const hash128: Uint8Array = blake.blake2b(bytesHash, null, 16);

  const out = [hash128[0], hash128[1], hash128[2], hash128[3]];

  for (let i = 4; i < 16; i += 4) {
    out[0] ^= hash128[i];
    out[1] ^= hash128[i + 1];
    out[2] ^= hash128[i + 2];
    out[3] ^= hash128[i + 3];
  }
  return new Uint8Array(out);
}
