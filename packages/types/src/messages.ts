import { FileChanges } from '@holochain/scaffolding';

export enum ClientEventType {
  ApplyChanges = 'ApplyChanges',
  ReadDir = 'ReadDir',
  AutomaticSetup = 'AutomaticSetup',
  Exit = 'Exit',
}

export interface ClientEvents {
  [ClientEventType.ApplyChanges]: (changes: FileChanges[]) => void;
  [ClientEventType.ReadDir]: () => { dirPath: string };
  [ClientEventType.AutomaticSetup]: (appName: string) => void;
  [ClientEventType.Exit]: () => void;
}

export interface ServerEvents {}
