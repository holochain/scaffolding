import { PatcherDirectory, findByPath, PatcherFile, PatcherNodeType } from '@patcher/types';
import { patchNpmDependency } from '@patcher/vue';
import camelCase from 'lodash-es/camelCase';
import kebabCase from 'lodash-es/kebabCase';
import snakeCase from 'lodash-es/snakeCase';
import upperFirst from 'lodash-es/upperFirst';

export function patchProfiles(happDir: PatcherDirectory, itemsSingular: string, itemsPlural: string): PatcherDirectory {
  const packageJson = findByPath(happDir, 'ui/package.json') as PatcherFile;

  packageJson.content = patchNpmDependency(packageJson, '@holochain-open-dev/profiles', '^0.0.8').content;
  packageJson.content = patchNpmDependency(packageJson, '@holochain-open-dev/context', '^0.0.3').content;
  packageJson.content = patchNpmDependency(packageJson, '@holochain-open-dev/cell-client', '^0.3.2').content;

  const demo = findByPath(happDir, 'ui/demo/index.html') as PatcherFile;
  demo.content = profilesDemo(itemsSingular, itemsPlural);

  const crateDirs = findByPath(happDir, 'crates') as PatcherDirectory;

  crateDirs.children['profiles'] = profilesZome();

  let dnaYaml = findByPath(happDir, 'workdir/dna/dna.yaml') as PatcherFile;
  dnaYaml.content = addZomeToDnaYaml(dnaYaml, 'profiles').content;

  const rootCargoToml = happDir.children['Cargo.toml'] as PatcherFile;
  rootCargoToml.content = addCrateToRootCargoToml(rootCargoToml, 'crates/profiles').content;

  return happDir;
}

export function addZomeToDnaYaml(dnaYaml: PatcherFile, zomeName: string): PatcherFile {
  return {
    type: PatcherNodeType.File,
    content: `${dnaYaml.content}
  - name: ${zomeName}
    bundled: ../../target/wasm32-unknown-unknown/release/${zomeName}.wasm`,
  };
}

export function addCrateToRootCargoToml(cargoToml: PatcherFile, zomePath: string): PatcherFile {
  const content = cargoToml.content.replace(
    'members = [',
    `members = [
  "${zomePath}",`,
  );

  return {
    type: PatcherNodeType.File,
    content,
  };
}

export function profilesZome(): PatcherDirectory {
  return {
    type: PatcherNodeType.Directory,
    children: {
      'Cargo.toml': {
        type: PatcherNodeType.File,
        content: `[package]
name = "profiles"
version = "0.0.1"
edition = "2018"

[lib]
name = "profiles"
crate-type = [ "cdylib", "rlib" ]

[dependencies]
hc_zome_profiles = {git = "https://github.com/holochain-open-dev/profiles", rev = "for-hc-v0.0.125"}
        `,
      },
      src: {
        type: PatcherNodeType.Directory,
        children: {
          'lib.rs': {
            type: PatcherNodeType.File,
            content: `extern crate hc_zome_profiles;`,
          },
        },
      },
    },
  };
}

export function profilesDemo(itemsSingular: string, itemsPlural: string): string {
  const createEl = `Create${upperFirst(camelCase(itemsSingular))}`;
  const createElTagName = `create-${kebabCase(itemsSingular)}`;
  const store = `${upperFirst(camelCase(itemsPlural))}Store`;
  const service = `${itemsPlural}Service`;
  const context = `${camelCase(itemsPlural)}StoreContext`;
  const testEl = `${upperFirst(camelCase(itemsPlural))}TestApp`;
  return `<!DOCTYPE html>
  <html lang="en-GB">
    <head>
      <meta charset="utf-8" />
      <link
        href="https://fonts.googleapis.com/css?family=Material+Icons&display=block"
        rel="stylesheet"
      />
  
      <link
        rel="stylesheet"
        href="https://fonts.googleapis.com/css?family=Roboto:300,400,500,700&display=swap"
      />
    </head>
  
    <link rel="stylesheet" href="styles.css" />
  
    <body>
      <${kebabCase(itemsPlural)}-test-app></${kebabCase(itemsPlural)}-test-app>
  
      <script type="module">
        import {
          ProfilesService,
          ProfilesStore,
          ProfilePrompt,
          SearchAgent,
          profilesStoreContext,
        } from '@holochain-open-dev/profiles';
  
        import { ${createEl}, ${store}, ${context} } from '../dist';
  
        import { ContextProvider } from '@holochain-open-dev/context';
        import { HolochainClient } from '@holochain-open-dev/cell-client';
        import { ScopedElementsMixin } from '@open-wc/scoped-elements';
        import { LitElement, html } from 'lit';
  
        class ${testEl} extends ScopedElementsMixin(LitElement) {
          static get properties() {
            return {
              loaded: {
                type: Boolean,
              },
            };
          }
  
          async firstUpdated() {
            const client = await HolochainClient.connect(
              \`ws://localhost:\${process.env.HC_PORT}\`,
              'test-app'
            );
            const cellClient = client.forCell(
              client.cellDataByRoleId('${snakeCase(itemsPlural)}')
            );
  
            const profilesStore = new ProfilesStore(cellClient);
            new ContextProvider(this, profilesStoreContext, profilesStore);
  
            new ContextProvider(
              this,
              ${context},
              new ${store}(cellClient)
            );
            this.loaded = true;
          }
  
          render() {
            if (!this.loaded) return html\`<span>Loading...</span>\`;
            return html\`
              <profile-prompt>
                <${createElTagName}></${createElTagName}>
              </profile-prompt>
            \`;
          }
  
          static get scopedElements() {
            return {
              'profile-prompt': ProfilePrompt,
              '${createElTagName}': ${createEl},
            };
          }
        }
  
        customElements.define('${itemsPlural}-test-app', ${testEl});
      </script>
    </body>
  </html>
  `;
}
