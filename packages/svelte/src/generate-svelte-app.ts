import { findByPath, ScDirectory, ScFile } from '@source-craft/types';
import { EntryDefinition, HappDefinition } from '@holochain-scaffolding/definitions';
import { generateTsTypesForHapp } from '@holochain-scaffolding/generators';
import { happVocabulary, elementsImports, happTsGenerators } from '@holochain-scaffolding/vocabulary';
import litApp from './app';
import { addComponentsForEntryDef } from './add-components';
import { addNpmDependency } from '@source-craft/npm';
import { camelCase, kebabCase, upperFirst } from 'lodash-es';

export function generateLitApp(happDefinition: HappDefinition): ScDirectory {
  const firstEntry = getFirstEntryDef(happDefinition);

  const firstType = firstEntry.entryDef.typeDefinition;

  const create = `Create${upperFirst(camelCase(firstType.name))}`;
  const detail = `${upperFirst(camelCase(firstType.name))}Detail`;

  let app = litApp({
    happName: happDefinition.name,
    subcomponentImports: `import ${create} from './components/${firstEntry.dna}/${firstEntry.zome}/${create}.svelte';
import ${detail} from './components/${firstEntry.dna}/${firstEntry.zome}/${detail}.svelte';`,
    appContent: `<${create} on:${kebabCase(firstType.name)}-created="{e => entryHash = e.detail.entryHash}"></${create}>

    {#if entryHash}
      <${detail} entryHash={entryHash}></${detail}>
    {/if}`,
  });
  const typesDir = generateTsTypesForHapp(happDefinition);

  (app.children['src'] as ScDirectory).children['types'] = typesDir;

  for (const dna of happDefinition.dnas) {
    for (const zome of dna.zomes) {
      for (const entryDef of zome.entry_defs) {
        app = addComponentsForEntryDef(
          app,
          happVocabulary,
          happTsGenerators,
          elementsImports,
          entryDef.typeDefinition,
          dna.name,
          zome.name,
        );
      }
    }
  }

  const packageJson = findByPath(app, 'package.json') as ScFile;
  packageJson.content = addNpmDependency(packageJson, '@material/mwc-button', '^0.25.3').content;
  packageJson.content = addNpmDependency(packageJson, '@material/mwc-circular-progress', '^0.25.3').content;

  return app;
}

function getFirstEntryDef(happDefinition: HappDefinition): { zome: string; dna: string; entryDef: EntryDefinition } {
  for (const dna of happDefinition.dnas) {
    for (const zome of dna.zomes) {
      for (const entryDef of zome.entry_defs) {
        return {
          dna: dna.name,
          zome: zome.name,
          entryDef,
        };
      }
    }
  }
  throw new Error('There are no entries in this happ');
}
