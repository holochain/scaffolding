import { ScFile, ScNodeType } from '@source-craft/types'; 

export const mainTs = (): ScFile => ({
  type: ScNodeType.File,
  content: `import { createApp } from 'vue'
import App from './App.vue'

createApp(App).mount('#app')
`
});
    