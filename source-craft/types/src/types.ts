export enum ScNodeType {
  Directory,
  File,
}

export interface ScDirectory {
  type: ScNodeType.Directory;
  children: Record<string, ScDirectory | ScFile>;
}

export interface ScFile {
  type: ScNodeType.File;

  content: string;
}

export type ScNode = ScDirectory | ScFile;
