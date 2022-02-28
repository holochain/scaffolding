import { TypeDefinition } from '@type-craft/vocabulary';
import { TypeElementsImports } from '@type-craft/elements-imports';
import { importDeclaration } from '@source-craft/web-apps';
export * from './generators'

export const contentType: TypeDefinition<string, void> = {
  name: 'Text Content',
  description: '',

  sample: () => `Lorem ipsum dolor sit amet, consectetur adipiscing elit. 
Maecenas aliquam, elit ac interdum gravida, leo odio accumsan augue, ut 
vehicula ex elit vel est. Phasellus rutrum tortor a nunc euismod malesuada.
Suspendisse potenti. Nulla aliquet, eros vitae feugiat vehicula, nibh odio mattis 
purus, ac interdum augue risus non justo. Maecenas sed volutpat urna. Aenean nec 
ante tellus. Mauris in urna ac lorem bibendum egestas tincidunt nec odio. Donec
sit amet elit nisl. Integer eleifend non ipsum rutrum viverra. Phasellus faucibus
arcu id dolor elementum volutpat. Donec tincidunt finibus nunc, et elementum erat
pellentesque id. Nam dictum rutrum pellentesque. Phasellus sollicitudin lectus
vitae lobortis elementum. Donec vestibulum quam eget accumsan hendrerit.`,
};

export const elementImports: TypeElementsImports = {
  create: {
    sideEffectImport: {
      importDeclaration: importDeclaration('@type-craft/text/create-content'),
      packageName: '@type-craft/text',
      version: '0.0.1',
    },
    tagName: 'create-content',
  },
  detail: {
    sideEffectImport: {
      importDeclaration: importDeclaration('@type-craft/text/content-detail'),
      packageName: '@type-craft/text',
      version: '0.0.1',
    },
    tagName: 'content-detail',
  },
};
