import { PatcherDirectory, PatcherFile, PatcherNode, PatcherNodeType } from '@patcher/types';
import camelCase from 'lodash-es/camelCase';
import uniq from 'lodash-es/uniq';
import flatten from 'lodash-es/flatten';
import toId from 'to-js-identifier';

import { escapeTemplateLiteral, replaceAll } from './utils';

export enum Case {
  TitleCase = 'TitleCase',
  SnakeCase = 'snake_case',
  CamelCase = 'camelCase',
  KebabCase = 'kebab-case',
}

export interface LiteralToTemplate {
  literal: string;
  template: string;
  caseByExtensions?: Record<string, Case>;
}

export function directoryToPatcher(
  directory: PatcherDirectory,
  templateLiterals: LiteralToTemplate[],
): PatcherDirectory {
  const [d, _] = innerDirectoryToPatcher(directory, templateLiterals);
  return d;
}

function innerDirectoryToPatcher(
  directory: PatcherDirectory,
  templateLiterals: LiteralToTemplate[],
): [PatcherDirectory, string[]] {
  const patcher: PatcherDirectory = {
    type: PatcherNodeType.Directory,
    children: {},
  };

  const literalsInFile: Record<string, string[]> = {};

  for (const [childPath, child] of Object.entries(directory.children)) {
    let newChildPath = childPath;
    for (const { literal, template } of templateLiterals) {
      if (newChildPath.includes(literal)) {
        newChildPath = replaceAll(childPath, literal, template);
      }
    }

    if (child.type === PatcherNodeType.Directory) {
      const [p, literals] = innerDirectoryToPatcher(child, templateLiterals);
      patcher.children[id(newChildPath)] = p;
      literalsInFile[childPath] = literals;
    } else {
      const [p, literals] = filePatcher(childPath, child, templateLiterals);
      patcher.children[`${id(newChildPath)}.ts`] = p;

      literalsInFile[childPath] = literals;
    }
  }

  const [dirPatcherFile, literalsInFolder] = dirPatcher(directory, templateLiterals, literalsInFile);
  patcher.children['index.ts'] = dirPatcherFile;
  const allLiterals = uniq(flatten([...Object.values(literalsInFile), ...literalsInFolder]));

  return [patcher, allLiterals];
}

function fileExtension(fileName: string): string {
  const components = fileName.split('.');

  return components[components.length - 1];
}

function casedLiteral(template: string, literalCase: Case | undefined): string {
  if (!literalCase) return `\${${template}}`;

  switch (literalCase) {
    case Case.TitleCase:
      return `\${upperFirst(camelCase(${template}))}`;
    case Case.CamelCase:
      return `\${camelCase(${template})}`;
    case Case.KebabCase:
      return `\${kebabCase(${template})}`;
    case Case.SnakeCase:
      return `\${snakeCase(${template})}`;
  }
}

export function filePatcher(
  name: string,
  file: PatcherFile,
  templateLiterals: LiteralToTemplate[],
): [PatcherFile, string[]] {
  const existingLiterals = [];

  const extension = fileExtension(name);

  let content = escapeTemplateLiteral(file.content);

  for (const { literal, template, caseByExtensions } of templateLiterals) {
    if (content.includes(literal)) {
      const caseForExt = caseByExtensions ? caseByExtensions[extension] : undefined;

      content = replaceAll(content, literal, casedLiteral(template, caseForExt));
      existingLiterals.push(literal);
    }
  }

  const varLiterals = existingLiterals.map(l => findTemplateByLiteral(templateLiterals, l));

  for (const { literal, template } of templateLiterals) {
    if (name.includes(literal)) {
      name = replaceAll(name, literal, template);
    }
  }

  return [
    {
      type: PatcherNodeType.File,
      content: `import { PatcherFile, PatcherNodeType } from '@patcher/types';
import camelCase from 'lodash-es/camelCase';
import kebabCase from 'lodash-es/kebabCase';
import upperFirst from 'lodash-es/upperFirst';
import snakeCase from 'lodash-es/snakeCase';

export const ${id(name)} = (${literalsParametersDef(varLiterals)}): PatcherFile => ({
  type: PatcherNodeType.File,
  content: \`${content}\`
});
    `,
    },
    varLiterals,
  ];
}

function findTemplateByLiteral(templateLiterals: LiteralToTemplate[], literalToFind: string): string {
  return templateLiterals.find(({ literal }) => literal === literalToFind).template;
}

function dirPatcher(
  directory: PatcherDirectory,
  templateLiterals: LiteralToTemplate[],
  literalsInChildren: Record<string, string[]>,
): [PatcherFile, string[]] {
  const children: Record<string, PatcherNode> = {};
  const literalsInModifiedChildren: Record<string, string[]> = {};
  const modifiedImports: Record<string, string> = {};

  const literalsInNames = [];

  for (let [childPath, child] of Object.entries(directory.children)) {
    let modifiedImport = childPath;
    const literalsInChild = literalsInChildren[childPath];
    let found = false;
    for (const { literal, template } of templateLiterals) {
      if (childPath.includes(literal)) {
        literalsInNames.push(template);
        childPath = replaceAll(childPath, literal, `\${${template}}`);
        modifiedImport = replaceAll(modifiedImport, literal, template);
        found = true;
      }
    }

    if (found) {
      childPath = `[\`${childPath}\`]`;
    } else {
      childPath = `'${childPath}'`;
    }

    children[childPath] = child;
    literalsInModifiedChildren[childPath] = literalsInChild;
    modifiedImports[childPath] = modifiedImport;
  }

  const allLiterals = uniq(flatten([...Object.values(literalsInModifiedChildren), ...literalsInNames]));

  return [
    {
      type: PatcherNodeType.File,
      content: `import { PatcherNodeType, PatcherDirectory } from '@patcher/types'; 

${Object.entries(children)
  .map(
    ([childPath, child]) =>
      `import ${
        child.type === PatcherNodeType.File ? `{ ${id(modifiedImports[childPath])} }` : id(modifiedImports[childPath])
      } from './${id(modifiedImports[childPath])}';`,
  )
  .join('\n')}  

export default (${literalsParametersDef(allLiterals)}): PatcherDirectory => ({
  type: PatcherNodeType.Directory,
  children: {
  ${Object.keys(children)
    .map(
      childPath =>
        `${childPath}: ${id(modifiedImports[childPath])}(${passParameters(literalsInModifiedChildren[childPath])})`,
    )
    .join(',\n  ')}
  }
})`,
    },
    allLiterals,
  ];
}

function literalsParametersDef(literals: string[]): string {
  if (literals.length === 0) return '';

  return `{${literals.join(', ')}}: {${literals.map(l => `${l}: string;`).join(' ')}}`;
}

function passParameters(literals: string[]): string {
  if (literals.length === 0) return '';

  return `{${literals.join(', ')}}`;
}

function id(str: string): string {
  return toId(camelCase(str));
}
