import { ScDirectory, ScFile } from '@source-craft/types';

export function patchCustomElementsSetup(dir: ScDirectory): ScDirectory {
  if (Object.keys(dir.children).includes('vite.config.ts')) {
    const viteConfigFile = dir.children['vite.config.ts'] as ScFile;

    const customElement = `template: {
      transformAssetUrls,
      compilerOptions: {
        // treat all tags with a dash as custom elements
        isCustomElement: (tag) => tag.includes("-"),
      },
    },`;

    if (!viteConfigFile.content.includes(customElement)) {
      viteConfigFile.content.replace(
        'vue({',
        `'vue({'
${customElement}`,
      );
    }
  }

  return dir;
}
