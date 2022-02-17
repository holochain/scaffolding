import { HappDefinition, DnaDefinition } from '@holochain-scaffolding/definitions';

import { dna } from '../dna';
import { defaultNix } from '../nix/default.nix';

import { workspaceCargoToml } from './Cargo.toml';

import { happYaml } from './happ.yaml';
import { tryoramaTests } from '../tryorama';
import { githubWorkfows } from '../github';
import { gitignore } from './gitignore';
import { readme } from './README.md';
import { PatcherDirectory, PatcherFile, PatcherNodeType } from '@patcher/types';
import { npmRc } from '../npm/npmrc';
import { rootPackageJson } from '../npm/package.json';

export function happ(happDef: HappDefinition): PatcherDirectory {
  const happDir: PatcherDirectory = {
    type: PatcherNodeType.Directory,
    children: {},
  };

  if (happDef.dnas.length < 2) {
    happDir.children['dna'] = dna(happDef, 0, '../../');
  } else {
    const dnasDir: PatcherDirectory = {
      type: PatcherNodeType.Directory,
      children: {},
    };
    for (const [dnaIndex, dnaDef] of happDef.dnas.entries()) {
      dnasDir.children[dnaDef.name] = dna(happDef, dnaIndex, '../../../');
    }
    happDir.children['dnas'] = dnasDir;
  }

  return {
    type: PatcherNodeType.Directory,
    children: {
      ...happDir.children,
      'default.nix': defaultNix('1cb431ac2d30d6f44dbcb5a40520f7328ae49ec1'),
      workdir: {
        type: PatcherNodeType.Directory,
        children: {
          'happ.yaml': happYaml(happDef),
        },
      },
      'Cargo.toml': workspaceCargoToml(happDef),
      'package.json': rootPackageJson(happDef),
      '.npmrc': npmRc(),
      '.github': githubWorkfows(),
      '.gitignore': gitignore(),
      'README.md': readme(happDef),
      tests: tryoramaTests(happDef),
    },
  };
}
