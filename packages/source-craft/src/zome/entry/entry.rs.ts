import { EntryDefinition } from '@holochain-scaffolding/definitions';
import { PatcherFile, PatcherNodeType } from '@source-craft/types';
import { generateTypesFile, ProgrammingLanguages } from '@type-craft/vocabulary';

export  function entryTypes(entryDef: EntryDefinition): PatcherFile {
  const content = generateTypesFile([entryDef.typeDefinition], ProgrammingLanguages.Rust);

  return {
    type: PatcherNodeType.File,
    content,
  };
}
