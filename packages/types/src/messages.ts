import { FileChanges } from '@holochain/scaffolding-generators';

export enum ClientEventType {
  ApplyChanges = 'ApplyChanges',
  ReadDir = 'ReadDir',
  AutomaticSetup = 'AutomaticSetup',
}

export interface ClientEvents {
  [ClientEventType.ApplyChanges]: (changes: FileChanges[]) => void;
  [ClientEventType.ReadDir]: () => { dirPath: string };
  [ClientEventType.AutomaticSetup]: (appName: string) => void;
}

export interface ServerEvents {}
