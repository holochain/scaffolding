import { ScDirectory, ScFile } from '@source-craft/types';

export type ScriptName = string;

export function patchEnvVars(
  dir: ScDirectory,
  varsByScript: Record<ScriptName, Record<string, string>>,
): ScDirectory {
  const packageJson = dir.children['package.json'] as ScFile;

  const content = JSON.parse(packageJson.content);

  const scripts = content.scripts;

  for (const [script, vars] of Object.entries(varsByScript)) {
    if (scripts[script]) {
      const varsStr = Object.entries(vars)
        .map(([varName, varValue]) => `${varName}=${varValue}`)
        .join(' ');
      scripts[script] = `${varsStr} ${scripts[script]}`;
    }
  }

  packageJson.content = JSON.stringify(content, null, 2);

  return dir;
}
