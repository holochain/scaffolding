import { findByPath, ScDirectory, ScFile } from '@source-craft/types';
import { HappDefinition } from '@holochain-scaffolding/definitions';
import { generateTsTypesForHapp } from '@holochain-scaffolding/generators';
import { happVocabulary, elementsImports, happTsGenerators } from '@holochain-scaffolding/vocabulary';
import vueApp from './app';
import { addComponentsForEntryDef } from './add-components';
import { addNpmDependency } from '@source-craft/npm';

export function generateVueApp(happDefinition: HappDefinition): ScDirectory {
  let app = vueApp({
    happName: happDefinition.name,
    appContent: '<!-- TODO: put here the content of your app -->',
    appSubcomponents: '// TODO: Add here the appropriate subcomponents',
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
