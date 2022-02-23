import { ScFile, ScNodeType } from '@source-craft/types';
import camelCase from 'lodash-es/camelCase';
import kebabCase from 'lodash-es/kebabCase';
import upperFirst from 'lodash-es/upperFirst';
import snakeCase from 'lodash-es/snakeCase';

export const rocketConfigJs = ({moduleNamePlural}: {moduleNamePlural: string;}): ScFile => ({
  type: ScNodeType.File,
  content: `import { rocketLaunch } from "@rocket/launch";
import commonjs from "@rollup/plugin-commonjs";
import resolve from "@rollup/plugin-node-resolve";
import { adjustPluginOptions, addPlugin } from "plugins-manager";

const resolveOptions = {
  browser: true,
  preferBuiltins: false,
  mainFields: ["module", "browser", "main"],
};

/** @type {import('@rocket/cli').RocketCliOptions} */
export default {
  setupDevAndBuildPlugins: [addPlugin(commonjs, {})],
  setupBuildPlugins: [adjustPluginOptions(resolve, resolveOptions)],
  presets: [rocketLaunch()],
  devServer: {
    nodeResolve: resolveOptions,
  },
  pathPrefix: "/${moduleNamePlural}/",
};
`
});
    