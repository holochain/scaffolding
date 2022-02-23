import { HappDefinition, EntryDefinition } from '@holochain-scaffolding/definitions';
import { PatcherDirectory, PatcherNodeType, PatcherFile } from '@source-craft/types';
import { patchNpmDependency, vueComponent } from '@source-craft/vue';
import { WebComponent } from '@source-craft/web-apps';
import { titleCase } from '../../utils';
import { createEntryWebComponent } from './create-entry';
import { readEntryWebComponent } from './read-entry';

export function addWebComponentsForHapp(rootDir: PatcherDirectory, happ: HappDefinition): PatcherDirectory {
  const src = rootDir.children['src'] as PatcherDirectory;
  const componentsDir = src.children['components'] as PatcherDirectory;

  for (const dna of happ.dnas) {
    const dnaDir: PatcherDirectory = {
      type: PatcherNodeType.Directory,
      children: {},
    };

    for (const zome of dna.zomes) {
      const zomeDir: PatcherDirectory = {
        type: PatcherNodeType.Directory,
        children: {},
      };

      for (const entry of zome.entry_defs) {
        const dir = webComponentsForEntry(dna.name, zome.name, entry);
        zomeDir.children = {
          ...zomeDir.children,
          ...dir.children,
        };

        if (entry.typeDefinition.fields) {
          for (const field of entry.typeDefinition.fields) {
            for (const createEl of field.type.create) {
              rootDir.children['package.json'] = patchNpmDependency(
                rootDir.children['package.json'] as PatcherFile,

                createEl.package,
                createEl.version,
              );
            }
            for (const detailEl of field.type.detail) {
              rootDir.children['package.json'] = patchNpmDependency(
                rootDir.children['package.json'] as PatcherFile,

                detailEl.package,
                detailEl.version,
              );
            }
          }
        }
      }

      dnaDir.children[zome.name] = zomeDir;
    }

    componentsDir.children[dna.name] = dnaDir;
  }

  rootDir.children['package.json'] = patchNpmDependency(
    rootDir.children['package.json'] as PatcherFile,
    '@material/mwc-button',
    '^0.25.3',
  );

  return rootDir;
}

export function webComponentsForEntry(dnaName: string, zomeName: string, entry: EntryDefinition): PatcherDirectory {
  const children: Record<string, PatcherFile> = {};

  children[`Create${titleCase(entry.typeDefinition.name)}.vue`] = vueComponent(
    createEntryWebComponent(dnaName, zomeName, entry.typeDefinition),
  );
  children[`Read${titleCase(entry.typeDefinition.name)}.vue`] = vueComponent(
    readEntryWebComponent(dnaName, zomeName, entry.typeDefinition),
  );

  return {
    type: PatcherNodeType.Directory,
    children,
  };
}
