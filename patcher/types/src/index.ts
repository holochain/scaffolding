export enum PatcherNodeType {
  Directory,
  File,
}

export interface PatcherDirectory {
  type: PatcherNodeType.Directory;
  children: Record<string, PatcherDirectory | PatcherFile>;
}

export interface PatcherFile {
  type: PatcherNodeType.File;

  content: string;
}

export type PatcherNode = PatcherDirectory | PatcherFile;
