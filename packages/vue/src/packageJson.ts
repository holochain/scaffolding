export const packageJson = () => `{
  "name": "ui",
  "version": "0.1.0",
  "private": true,
  "scripts": {
    "start": "VUE_APP_HC_PORT=\$HC_PORT vue-cli-service serve --open",
    "build": "VUE_APP_HC_PORT=undefined vue-cli-service build",
    "lint": "vue-cli-service lint"
  },
  "dependencies": {
    "@holochain/client": "^0.3.2",
    "core-js": "^3.6.5",
    "vue": "^3.0.0",
    "vue-router": "^4.0.0-0",
    "vuex": "^4.0.0-0"
  },
  "devDependencies": {
    "@typescript-eslint/eslint-plugin": "^4.18.0",
    "@typescript-eslint/parser": "^4.18.0",
    "@vue/cli-plugin-babel": "~4.5.0",
    "@vue/cli-plugin-eslint": "~4.5.0",
    "@vue/cli-plugin-router": "~4.5.0",
    "@vue/cli-plugin-typescript": "~4.5.0",
    "@vue/cli-plugin-vuex": "~4.5.0",
    "@vue/cli-service": "~4.5.0",
    "@vue/compiler-sfc": "^3.0.0",
    "@vue/eslint-config-typescript": "^7.0.0",
    "eslint": "^6.7.2",
    "eslint-plugin-vue": "^7.0.0",
    "lint-staged": "^9.5.0",
    "run-singleton-cli": "^0.0.5",
    "typescript": "~4.1.5"
  },
  "gitHooks": {
    "pre-commit": "lint-staged"
  },
  "lint-staged": {
    "*.{js,jsx,vue,ts,tsx}": [
      "vue-cli-service lint",
      "git add"
    ]
  }
}
`
    