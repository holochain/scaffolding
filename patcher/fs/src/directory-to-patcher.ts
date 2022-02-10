import { PatcherDirectory, PatcherFile, PatcherNodeType } from '@patcher/types';
import camelCase from 'lodash-es/camelCase';
import uniq from 'lodash-es/uniq';
import flatten from 'lodash-es/flatten';
import toId from 'to-js-identifier';

import { escapeTemplateLiteral, replaceAll } from './utils';

export function directoryToPatcher(
  directory: PatcherDirectory,
  templateLiterals: Record<string, string>,
): PatcherDirectory {
  const [d, _] = innerDirectoryToPatcher(directory, templateLiterals);
  return d;
}

function innerDirectoryToPatcher(
  directory: PatcherDirectory,
  templateLiterals: Record<string, string>,
): [PatcherDirectory, string[]] {
  const patcher: PatcherDirectory = {
    type: PatcherNodeType.Directory,
    children: {},
  };

  const literalsInFile: Record<string, string[]> = {};

  for (const [childPath, child] of Object.entries(directory.children)) {
    if (child.type === PatcherNodeType.Directory) {
      const [p, literals] = innerDirectoryToPatcher(child, templateLiterals);
      patcher.children[camelCase(childPath)] = p;
      literalsInFile[childPath] = literals;
    } else {
      const [p, literals] = filePatcher(childPath, child, templateLiterals);
      patcher.children[`${camelCase(childPath)}.ts`] = p;

      literalsInFile[childPath] = literals;
    }
  }

  patcher.children['index.ts'] = dirPatcher(directory, literalsInFile);
  const allLiterals = uniq(flatten(Object.values(literalsInFile)));

  return [patcher, allLiterals];
}

export function filePatcher(
  name: string,
  file: PatcherFile,
  templateLiterals: Record<string, string>,
): [PatcherFile, string[]] {
  const existingLiterals = Object.keys(templateLiterals).filter(t => file.content.includes(t));

  let content = escapeTemplateLiteral(file.content);

  for (const literal of existingLiterals) {
    content = replaceAll(content, literal, `\${${templateLiterals[literal]}}`);
  }

  const varLiterals = existingLiterals.map(l => templateLiterals[l]);

  return [
    {
      type: PatcherNodeType.File,
      content: `import { PatcherFile, PatcherNodeType } from '@patcher/types'; 

export const ${toId(camelCase(name))} = (${literalsParametersDef(varLiterals)}): PatcherFile => ({
  type: PatcherNodeType.File,
  content: \`${content}\`
});
    `,
    },
    varLiterals,
  ];
}

function dirPatcher(directory: PatcherDirectory, literalsInFile: Record<string, string[]>): PatcherFile {
  const allLiterals = uniq(flatten(Object.values(literalsInFile)));

  return {
    type: PatcherNodeType.File,
    content: `import { PatcherNodeType, PatcherDirectory } from '@patcher/types'; 

${Object.entries(directory.children)
  .map(
    ([childPath, child]) =>
      `import ${
        child.type === PatcherNodeType.File ? `{ ${toId(camelCase(childPath))} }` : toId(camelCase(childPath))
      } from './${camelCase(childPath)}';`,
  )
  .join('\n')}  

export default (${literalsParametersDef(allLiterals)}): PatcherDirectory => ({
  type: PatcherNodeType.Directory,
  children: {
  ${Object.keys(directory.children)
    .map(child => `'${child}': ${toId(camelCase(child))}(${passParameters(literalsInFile[child])})`)
    .join(',\n  ')}
  }
})`,
  };
}

function literalsParametersDef(literals: string[]): string {
  if (literals.length === 0) return '';

  return `{${literals.join(', ')}}: {${literals.map(l => `${l}: string;`).join(' ')}}`;
}

function passParameters(literals: string[]): string {
  if (literals.length === 0) return '';

  return `{${literals.join(', ')}}`;
}
