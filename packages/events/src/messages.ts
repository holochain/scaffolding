import { ScDirectory } from '@source-craft/types';

export enum ClientEventType {
  WriteDirectory = 'WriteDirectory',
  ReadDir = 'ReadDir',
  AutomaticSetup = 'AutomaticSetup',
  Exit = 'Exit',
}

export interface ClientEvents {
  [ClientEventType.WriteDirectory]: (changes: { happ: ScDirectory; happName: string }) => void;
  [ClientEventType.ReadDir]: () => { dirPath: string };
  [ClientEventType.AutomaticSetup]: (appName: string) => void;
  [ClientEventType.Exit]: () => void;
}

export interface ServerEvents {}
