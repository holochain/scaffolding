import { EntryDefinition } from '@holochain-scaffolding/definitions';
import { PatcherFile, PatcherNodeType } from '@patcher/types';
import { generateTypesFile, ProgrammingLanguages } from '@typecraft/type-definition';

export  function entryTypes(entryDef: EntryDefinition): PatcherFile {
  const content = generateTypesFile([entryDef.typeDefinition], ProgrammingLanguages.Rust);

  return {
    type: PatcherNodeType.File,
    content,
  };
}
