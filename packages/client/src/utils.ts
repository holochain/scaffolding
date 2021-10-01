import { FileChanges, FileChangesType } from '@holochain/scaffolding-generators';
import JSZip, { JSZipObject } from 'jszip';

//@ts-ignore
import JSZipUtils from 'jszip-utils';

export function getUiTemplate(uiTemplate: string): Promise<FileChanges[]> {
  return new Promise((resolve, reject) => {
    JSZipUtils.getBinaryContent(`/templates/${uiTemplate}.zip`, function(err: any, data: any) {
      if (err) {
        reject(err);
        return;
      }

      JSZip.loadAsync(data)
        .then(d => zipToFileChanges(d))
        .then(fc => resolve(fc))
        .catch(reject);
    });
  });
}

export async function zipToFileChanges(jsZip: JSZip): Promise<FileChanges[]> {
  const fileChanges: FileChanges[] = [];

  for (const [name, object] of Object.entries(jsZip.files)) {
    if (!object.dir) {
      await addFile(name.split('/'), fileChanges, object);
    }
  }

  return fileChanges;
}

async function addFile(path: string[], fileChanges: FileChanges[], object: JSZipObject) {
  let changes = fileChanges;
  while (path.length > 0) {
    const [dirName] = path.splice(0, 1);

    if (path.length === 0) {
      // This is a file
      changes.push({
        type: FileChangesType.Create,
        fileName: dirName,
        content: await object.async('text'),
      });
    } else {
      // This is a dir
      let nestedChanges = changes.find(ch => ch.type === FileChangesType.InDir && ch.dirName === dirName);
      if (!nestedChanges) {
        nestedChanges = {
          type: FileChangesType.InDir,
          dirName,
          changes: [],
        };
        changes.push(nestedChanges);
      }
      changes = (nestedChanges as {
        type: FileChangesType.InDir;
        dirName: string;
        changes: FileChanges[];
      }).changes;
    }
  }
}
