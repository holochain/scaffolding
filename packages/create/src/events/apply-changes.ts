import { FileChanges, FileChangesType } from '@holochain/rad-generators';
import fs from 'fs';

export function applyGeneratedChanges(baseDirPath: string, fileChanges: FileChanges[]) {
  createDirIfNotExists(baseDirPath);
  
  for (const change of fileChanges) {
    if (change.type === FileChangesType.Create) {
      fs.writeFileSync(`${baseDirPath}/${change.fileName}`, change.content);
    } else if (change.type === FileChangesType.InDir) {
      const dirPath = `${baseDirPath}/${change.dirName}`;
      createDirIfNotExists(dirPath);
      applyGeneratedChanges(dirPath, change.changes);
    }
  }
}

export function createDirIfNotExists(dir: string) {
  if (!fs.existsSync(dir)) {
    fs.mkdirSync(dir);
  }
}
