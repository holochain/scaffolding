import { PatcherDirectory, PatcherFile } from '@patcher/types';

export function patchCustomElementsSetup(dir: PatcherDirectory): PatcherDirectory {
  if (Object.keys(dir.children).includes('vite.config.ts')) {
    const viteConfigFile = dir.children['vite.config.ts'] as PatcherFile;

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
