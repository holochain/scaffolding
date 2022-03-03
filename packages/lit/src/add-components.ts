import { addNpmDependency } from '@source-craft/npm';
import { findByPath, ScDirectory, ScFile, ScNodeType } from '@source-craft/types';
import { getAllImports, VocabularyElementsImportDeclarations } from '@type-craft/web-components';
import { VocabularyTypescriptGenerators } from '@type-craft/typescript';
import { getAllChildrenTypes, TypeDefinition, Vocabulary } from '@type-craft/vocabulary';
import { camelCase, flatten, snakeCase, upperFirst } from 'lodash-es';

import { generateTypeDetailVueComponent } from './detail-type-component';
import { generateCreateTypeVueComponent } from './create-type-component';

const titleCase = (str: string) => upperFirst(camelCase(str));

export function addComponentsForEntryDef(
  litApp: ScDirectory,
  vocabulary: Vocabulary,
  typescriptGenerators: VocabularyTypescriptGenerators,
  elementsImports: VocabularyElementsImportDeclarations,
  type: TypeDefinition<any, any>,
  dnaName: string,
  zomeName: string,
): ScDirectory {
  const srcDir = findByPath(litApp, 'src') as ScDirectory;

  const componentsDir: ScDirectory = {
    type: ScNodeType.Directory,
    children: {},
  };

  srcDir.children['components'] = componentsDir;

  let dnaComponentsDir = findByPath(componentsDir, dnaName) as ScDirectory;

  if (!dnaComponentsDir) {
    dnaComponentsDir = {
      type: ScNodeType.Directory,
      children: {},
    };
    componentsDir.children[dnaName] = dnaComponentsDir;
  }

  let zomeComponentsDir = findByPath(dnaComponentsDir, zomeName) as ScDirectory;

  if (!zomeComponentsDir) {
    zomeComponentsDir = {
      type: ScNodeType.Directory,
      children: {},
    };
    dnaComponentsDir.children[zomeName] = zomeComponentsDir;
  }

  const createComponentFile = generateCreateTypeVueComponent(
    typescriptGenerators,
    elementsImports,
    type,
    dnaName,
    zomeName,
  );

  const detailComponentFile = generateTypeDetailVueComponent(
    typescriptGenerators,
    elementsImports,
    type,
    dnaName,
    zomeName,
  );

  zomeComponentsDir.children[`create-${snakeCase(type.name)}.ts`] = createComponentFile;
  zomeComponentsDir.children[`${snakeCase(type.name)}-detail.ts`] = detailComponentFile;

  const packageJson = findByPath(litApp, 'package.json') as ScFile;

  const vocabularyForThisHapp: Vocabulary = {
    ...vocabulary,
    [type.name]: type,
  };

  const allTypes = getAllChildrenTypes(vocabularyForThisHapp, type.name);

  const allRenderers = allTypes.map(t => elementsImports[t]).filter(r => !!r);
  const allImports = flatten(allRenderers.map(r => getAllImports(r)));

  for (const i of allImports) {
    packageJson.content = addNpmDependency(packageJson, i.packageName, i.version).content;
  }

  return litApp;
}
