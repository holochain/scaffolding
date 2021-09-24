import { Change } from 'diff';

export enum FileChangesType {
  InDir = 'InDir',
  Create = 'Create',
  Modify = 'Modify',
  Delete = 'Delete',
}

export type FileChanges =
  | {
      type: FileChangesType.InDir;
      dirName: string;
      changes: FileChanges[];
    }
  | {
      type: FileChangesType.Create;
      fileName: string;
      content: string;
    }
  | {
      type: FileChangesType.Modify;
      fileName: string;
      changesToFile: Change[];
    }
  | {
      type: FileChangesType.Delete;
      fileName: string;
    };

export interface HappDefinition {
  name: string;
  dnas: DnaDefinition[];
}

export interface DnaDefinition {
  name: string;
  zomes: ZomeDefinition[];
}

export interface ZomeDefinition {
  name: string;
}
