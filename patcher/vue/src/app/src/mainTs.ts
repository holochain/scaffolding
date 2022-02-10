import { PatcherFile, PatcherNodeType } from '@patcher/types'; 

export const mainTs = (): PatcherFile => ({
  type: PatcherNodeType.File,
  content: `import { createApp } from 'vue'
import App from './App.vue'

createApp(App).mount('#app')
`
});
    