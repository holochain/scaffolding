import { lstatSync } from 'fs';

export function isDirectory(path: string): boolean {
  return lstatSync(path).isDirectory();
}

export function escapeTemplateLiteral(templateLiteral: string): string {
  const r1 = replaceAll(templateLiteral, '\\', '\\\\`');
  const r2 = replaceAll(r1, '`', '\\`');
  return replaceAll(r2, '$', '\\$');
}

export function replaceAll(str: string, find: string, replace: string) {
  return str.replace(new RegExp(escapeRegExp(find), 'g'), replace);
}
function escapeRegExp(string) {
  return string.replace(/[.*+?^${}()|[\]\\]/g, '\\$&'); // $& means the whole matched string
}
