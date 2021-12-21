import { Change } from 'diff';

export enum FileChangesType {
  InDir = 'InDir',
  Create = 'Create',
  Modify = 'Modify',
  Delete = 'Delete',
}

export type InDir = {
  type: FileChangesType.InDir;
  dirName: string;
  changes: FileChanges[];
};
export type CreateFile = {
  type: FileChangesType.Create;
  fileName: string;
  content: string;
};

export type ModifyFile = {
  type: FileChangesType.Modify;
  fileName: string;
  changesToFile: Change[];
};

export type DeleteFile = {
  type: FileChangesType.Delete;
  fileName: string;
};

export type FileChanges = InDir | CreateFile | ModifyFile | DeleteFile;
