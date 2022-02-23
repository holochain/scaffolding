import '@lit-labs/ssr/lib/render-with-global-dom-shim.js';

import test from 'tape';
import path from 'path';
import { applyPatch } from '@source-craft/fs';
import { holochainEntryTypeDefinition } from '@holochain-scaffolding/definitions';
import { dateType } from '@type-craft/date';
import { execSync } from 'child_process';

import { fileURLToPath } from 'url';

// @ts-ignore
const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

test('generate a full blown module', async t => {
  execSync('node dist/app.js resource-bookings');
});
