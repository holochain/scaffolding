import { ScDirectory } from '@source-craft/types';

export enum ClientEventType {
  ApplyPatch = 'ApplyPatch',
  ReadDir = 'ReadDir',
  AutomaticSetup = 'AutomaticSetup',
  Exit = 'Exit',
}

export interface ClientEvents {
  [ClientEventType.ApplyPatch]: (changes: { happ: ScDirectory; happName: string }) => void;
  [ClientEventType.ReadDir]: () => { dirPath: string };
  [ClientEventType.AutomaticSetup]: (appName: string) => void;
  [ClientEventType.Exit]: () => void;
}

export interface ServerEvents {}
