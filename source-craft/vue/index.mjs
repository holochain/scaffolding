import fs from 'fs';
import { readFolder, directoryTo@holochain-scaffolding/source-craft, applyPatch } from '@source-craft/fs';
import { fileURLToPath } from 'url';
import path from 'path';
const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

const d = readFolder(`${__dirname}/template`);

const patched = directoryToGenerator(d, {});

if (!fs.existsSync('./src')) fs.mkdirSync('./src');

applyPatch(`${__dirname}/src/app`, patched);
