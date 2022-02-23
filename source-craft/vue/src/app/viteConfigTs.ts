import { ScFile, ScNodeType } from '@source-craft/types'; 

export const viteConfigTs = (): ScFile => ({
  type: ScNodeType.File,
  content: `import { defineConfig } from 'vite';
import vue from '@vitejs/plugin-vue';

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [vue()],
  server: {
    open: true,
  },
});
`
});
    