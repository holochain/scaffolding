import { HappDefinition, ZomeDefinition } from '@holochain/rad-definitions';

import { FileChanges, FileChangesType } from '../file-changes';

import cargoToml from './Cargo.toml';
import { generateEntryDef } from './entry';
import libRs from './lib.rs';

export * from './entry';

export function generateZomeCargoToml(zomeName: string, author: string, hdkVersion = '0.0.120'): FileChanges[] {
  return [
    {
      type: FileChangesType.Create,
      fileName: `Cargo.toml`,
      content: cargoToml({
        zomeName: zomeName,
        author,
        hdkVersion,
      }),
    },
  ];
}

export async function generateZomeCode(zomeDefinition: ZomeDefinition): Promise<FileChanges[]> {
  const promises = zomeDefinition.entry_defs.map(
    async entryDef =>
      ({
        type: FileChangesType.InDir,
        dirName: entryDef.name,
        changes: await generateEntryDef(entryDef),
      } as FileChanges),
  );

  const entryDefs: FileChanges[] = await Promise.all(promises);
  return [
    {
      type: FileChangesType.InDir,
      dirName: 'src',
      changes: [
        {
          type: FileChangesType.Create,
          fileName: `lib.rs`,
          content: libRs(zomeDefinition),
        },
        ...entryDefs,
      ],
    },
  ];
}

export async function generateZome(happ: HappDefinition, dnaIndex: number, zomeIndex: number): Promise<FileChanges[]> {
  const crateName = getCrateName(happ, dnaIndex, zomeIndex);
  const zome = happ.dnas[dnaIndex].zomes[zomeIndex];

  return [...generateZomeCargoToml(crateName, '<AUTHOR>'), ...(await generateZomeCode(zome))];
}

export function getCrateName(happ: HappDefinition, dnaIndex: number, zomeIndex: number): string {
  let thereIsAnotherZomeInAnotherDnaWithTheSameName = false;
  const zome = happ.dnas[dnaIndex].zomes[zomeIndex];

  for (let i = 0; i < happ.dnas.length; i++) {
    const dna = happ.dnas[i];
    for (let j = 0; j < dna.zomes.length; j++) {
      if (i !== dnaIndex || j !== zomeIndex) {
        if (dna.zomes[j].name === zome.name) {
          thereIsAnotherZomeInAnotherDnaWithTheSameName = true;
        }
      }
    }
  }

  if (thereIsAnotherZomeInAnotherDnaWithTheSameName) {
    return `${happ.dnas[dnaIndex].name}_${zome.name}`;
  } else {
    return zome.name;
  }
}
