import { PatcherFile, PatcherNodeType } from '@patcher/types'; 

export const viteConfigTs = (): PatcherFile => ({
  type: PatcherNodeType.File,
  content: `import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [vue()]
})
`
});
    