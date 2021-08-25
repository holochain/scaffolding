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
