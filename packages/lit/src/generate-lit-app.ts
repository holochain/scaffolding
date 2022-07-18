import { findByPath, ScDirectory, ScFile } from '@source-craft/types';
import { EntryDefinition, HappDefinition } from '@holochain-scaffolding/definitions';
import { generateTsTypesForHapp } from '@holochain-scaffolding/generators';
import { happVocabulary, elementsImports, happTsGenerators } from '@holochain-scaffolding/vocabulary';
import litApp from './app';
import { addComponentsForEntryDef } from './add-components';
import { addNpmDependency } from '@source-craft/npm';
import { kebabCase } from 'lodash-es';

export function generateLitApp(happDefinition: HappDefinition): ScDirectory {
  const firstEntry = getFirstEntryDef(happDefinition);

  const firstType = firstEntry.entryDef.typeDefinition;

  const create = `create-${kebabCase(firstType.name)}`;
  const detail = `${kebabCase(firstType.name)}-detail`;

  let app = litApp({
    happName: happDefinition.name,
    subcomponentImports: `import './components/${firstEntry.dna}/${firstEntry.zome}/${create}';
import './components/${firstEntry.dna}/${firstEntry.zome}/${detail}';`,
    appContent: `<${create} @${kebabCase(
      firstType.name,
    )}-created=\${(e: CustomEvent) => this.entryHash = e.detail.entryHash}></${create}>
    \${this.entryHash ? html\`
      <${detail} .entryHash=\${this.entryHash}></${detail}>
    \` : html\`\`}`,
  });
  const typesDir = generateTsTypesForHapp(happDefinition);

  (app.children['src'] as ScDirectory).children['types'] = typesDir;

  for (const dna of happDefinition.dnas) {
    for (const coordinatorZome of dna.coordinator_zomes) {
      const integrityZome = dna.integrity_zomes.find(iz => coordinatorZome.dependencies.includes(iz.name));
      for (const entryDef of integrityZome.entry_defs) {
        app = addComponentsForEntryDef(
          app,
          happVocabulary,
          happTsGenerators,
          elementsImports,
          entryDef.typeDefinition,
          dna.name,
          coordinatorZome.name,
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
    for (const coordinatorZome of dna.coordinator_zomes) {
      const integrityZome = dna.integrity_zomes.find(iz => coordinatorZome.dependencies.includes(iz.name));

      for (const entryDef of integrityZome.entry_defs) {
        return {
          dna: dna.name,
          zome: coordinatorZome.name,
          entryDef,
        };
      }
    }
  }
  throw new Error('There are no entries in this happ');
}
