import { EntryDefinition, HappDefinition } from "@holochain-scaffolding/definitions";
import { PatcherDirectory } from "@patcher/types";
import { WebComponent } from "@patcher/web-apps";
import { TypeDefinition } from "@typecraft/type-definition";

export function addWebComponentsForEntries(dir: PatcherDirectory, happ: HappDefinition): PatcherDirectory {



  return dir;
}

export function addWebComponentsForEntry(dir: PatcherDirectory, entry: EntryDefinition): PatcherDirectory {

  const wb: WebComponent = {
    template: 
  };

  entry.typeDefinition.fields

  return dir;
}

export function createWebComponent(type: TypeDefinition<any, any>): WebComponent {
  const template = createTemplate(type);

  return {
    template,
    imports,
    localState,
    inject: {
      
    }
  }
}

export function createTemplate(type: TypeDefinition<any, any>) {
  type.fields
}