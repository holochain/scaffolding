import { ScFile, ScNodeType } from '@source-craft/types'; 

export const packageJson = (): ScFile => ({
  type: ScNodeType.File,
  content: `{
  "name": "vite-project",
  "version": "0.0.0",
  "scripts": {
    "start": "vite",
    "build": "vue-tsc --noEmit && vite build",
    "preview": "vite preview"
  },
  "dependencies": {
    "vue": "^3.2.25"
  },
  "devDependencies": {
    "@vitejs/plugin-vue": "^2.0.0",
    "typescript": "^4.4.4",
    "vite": "^2.7.2",
    "vue-tsc": "^0.29.8"
  }
}`
});
    