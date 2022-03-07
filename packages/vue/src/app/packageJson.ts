import { ScFile, ScNodeType } from '@source-craft/types';
import camelCase from 'lodash-es/camelCase';
import kebabCase from 'lodash-es/kebabCase';
import upperFirst from 'lodash-es/upperFirst';
import snakeCase from 'lodash-es/snakeCase';

export const packageJson = ({happName}: {happName: string;}): ScFile => ({
  type: ScNodeType.File,
  content: `{
  "name": "${happName}",
  "version": "0.0.0",
  "scripts": {
    "start": "VITE_HC_PORT=\$HC_PORT vite",
    "build": "vue-tsc --noEmit && vite build",
    "preview": "vite preview",
    "package": "npm run build && cd dist && bestzip ../dist.zip *"
  },
  "dependencies": {
    "@holochain/client": "^0.3.2",
    "@material/mwc-button": "^0.25.3",
    "@material/mwc-circular-progress": "^0.25.3",
    "@webcomponents/scoped-custom-element-registry": "0.0.4",
    "@types/ws": "^8.5.1",
    "vue": "^3.2.25"
  },
  "devDependencies": {
    "@vitejs/plugin-vue": "^2.0.0",
    "bestzip": "^2.2.0",
    "typescript": "^4.4.4",
    "vite": "^2.7.2",
    "vue-tsc": "^0.29.8"
  }
}
`
});
    