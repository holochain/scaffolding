import { FileChanges } from '@holochain/create-generators';

export enum ClientEventType {
  ApplyChanges = 'ApplyChanges',
  ReadDir = 'ReadDir',
}

export interface ClientEvents {
  [ClientEventType.ApplyChanges]: (changes: FileChanges[]) => void;
  [ClientEventType.ReadDir]: () => { dirPath: string };
}

export interface ServerEvents {}
