import { HappDefinition } from '@holochain/rad-definitions';

import { FileChanges, FileChangesType } from '../file-changes';
import { generateDna } from '../dna';
import { generateNixFile } from '../nix';

import cargoToml from './Cargo.toml';

import happYaml from './happ.yaml';
import { generateRootPackageJson } from '../npm';
import { generateTryorama } from '../tryorama';
import { generateGithubWorkfows } from '../github';
import gitignore from './gitignore';
import readme from './README.md';

export async function generateHapp(happ: HappDefinition): Promise<FileChanges[]> {
  return [
    ...generateNixFile(),
    ...generateHappWorkdir(happ),
    ...generateWorkspaceCargoToml(happ),
    ...(await generateDnas(happ)),
    ...generateRootPackageJson(happ),
    ...generateGithubWorkfows(happ),
    ...generateGitIgnore(),
    ...generateReadme(happ),
    {
      type: FileChangesType.InDir,
      dirName: 'tests',
      changes: generateTryorama(happ),
    },
  ];
}

export function generateHappYaml(happ: HappDefinition): FileChanges[] {
  return [
    {
      type: FileChangesType.Create,
      fileName: 'happ.yaml',
      content: happYaml(happ),
    },
  ];
}

export function generateGitIgnore(): FileChanges[] {
  return [
    {
      type: FileChangesType.Create,
      fileName: '.gitignore',
      content: gitignore(),
    },
  ];
}
export function generateReadme(happ: HappDefinition): FileChanges[] {
  return [
    {
      type: FileChangesType.Create,
      fileName: 'README.md',
      content: readme(happ),
    },
  ];
}

export function generateHappWorkdir(happ: HappDefinition): FileChanges[] {
  return [
    {
      type: FileChangesType.InDir,
      dirName: 'workdir',
      changes: generateHappYaml(happ),
    },
  ];
}

async function generateDnas(happ: HappDefinition): Promise<FileChanges[]> {
  if (happ.dnas.length === 1)
    return [
      {
        type: FileChangesType.InDir,
        dirName: 'dna',
        changes: await generateDna(happ.dnas[0], '../'),
      },
    ];

  const promises = happ.dnas.map(
    async dna =>
      ({
        type: FileChangesType.InDir,
        dirName: dna.name,
        changes: await generateDna(dna, '../../'),
      } as FileChanges),
  );

  const dnaChanges = await Promise.all(promises);

  return [
    {
      type: FileChangesType.InDir,
      dirName: 'dnas',
      changes: dnaChanges,
    },
  ];
}

export function generateWorkspaceCargoToml(happ: HappDefinition): FileChanges[] {
  return [
    {
      type: FileChangesType.Create,
      fileName: 'Cargo.toml',
      content: cargoToml(happ),
    },
  ];
}
