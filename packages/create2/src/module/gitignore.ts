import { PatcherFile, PatcherNodeType } from '@patcher/types';
import camelCase from 'lodash-es/camelCase';
import kebabCase from 'lodash-es/kebabCase';
import upperFirst from 'lodash-es/upperFirst';
import snakeCase from 'lodash-es/snakeCase';

export const gitignore = (): PatcherFile => ({
  type: PatcherNodeType.File,
  content: `target/
node_modules/
dist/
.cargo
.hc*
*.dna
*.happ
.running
docs/_merged_data/
docs/_merged_assets/
docs/_merged_includes/
/_site-dev/
/_site/`
});
    