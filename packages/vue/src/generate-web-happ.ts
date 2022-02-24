import { ScDirectory } from '@source-craft/types';
import { HappDefinition } from '@holochain-scaffolding/definitions';
import { webHapp, generateTsTypesForHapp } from '@holochain-scaffolding/generators';
import { happVocabulary, renderersImports, happTsGenerators } from '@holochain-scaffolding/vocabulary';
import generateVueApp from './app';
import { addCreateTypeComponent } from './create-type-component';

export function generateVueWebHapp(happDefinition: HappDefinition): ScDirectory {
  let vueApp = generateVueApp({
    happName: happDefinition.name,
  });
  const typesDir = generateTsTypesForHapp(happDefinition);

  (vueApp.children['src'] as ScDirectory).children['types'] = typesDir;

  for (const dna of happDefinition.dnas) {
    for (const zome of dna.zomes) {
      for (const entryDef of zome.entry_defs) {
        vueApp = addCreateTypeComponent(
          vueApp,
          happVocabulary,
          happTsGenerators,
          renderersImports,
          entryDef.typeDefinition,
          dna.name,
          zome.name,
        );
      }
    }
  }

  return webHapp(happDefinition, vueApp);
}
