import { createHandlerFnName } from '@holochain-scaffolding/patcher';
/* import { HappDefinition } from '@holochain-scaffolding/definitions';
import JSZip, { JSZipObject } from 'jszip';

//@ts-ignore
import JSZipUtils from 'jszip-utils';

export function getUiTemplate(
  uiTemplate: string,
  textModifier: (text: string) => string,
): Promise<FileChanges[]> {
  return new Promise((resolve, reject) => {
    JSZipUtils.getBinaryContent(`/templates/${uiTemplate}.zip`, function(err: any, data: any) {
      if (err) {
        reject(err);
        return;
      }

      JSZip.loadAsync(data)
        .then(d => zipToFileChanges(d, textModifier))
        .then(fc => resolve(fc))
        .catch(reject);
    });
  });
}

export async function zipToFileChanges(jsZip: JSZip, textModifier: (text: string) => string): Promise<FileChanges[]> {
  const fileChanges: FileChanges[] = [];

  for (const [name, object] of Object.entries(jsZip.files)) {
    if (!object.dir) {
      await addFile(name.split('/'), fileChanges, object, textModifier);
    }
  }

  return fileChanges;
}

async function addFile(
  path: string[],
  fileChanges: FileChanges[],
  object: JSZipObject,
  textModifier: (text: string) => string,
) {
  let changes = fileChanges;
  while (path.length > 0) {
    const [dirName] = path.splice(0, 1);

    if (path.length === 0) {
      // This is a file
      changes.push({
        type: FileChangesType.Create,
        fileName: dirName,
        content: textModifier(await object.async('text')),
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

const toReplace = (s: string) => `HC_SCAFFOLDING{${s}}`;

export interface ReplaceTargets {
  installedAppId: string;
  zomeName: string;
}

export function replaceText(text: string, target: ReplaceTargets): string {
  for (const [key, value] of Object.entries(target)) {
    text = text.replace(toReplace(key), value);
  }

  return text;
}
 */

import { HappDefinition } from '@holochain-scaffolding/definitions';

export function getFirstEntry(
  happ: HappDefinition,
): { sample: any; fnName: string; dnaName: string; zomeName: string; entryDefName: string } | undefined {
  for (const dna of happ.dnas) {
    for (const zome of dna.zomes) {
      for (const entryDef of zome.entry_defs) {
        if (entryDef.create) {
          return {
            dnaName: dna.name,
            zomeName: zome.name,
            fnName: createHandlerFnName(entryDef.name),
            sample: entryDef.sample,
            entryDefName: entryDef.name,
          };
        }
      }
    }
  }

  return undefined;
}
