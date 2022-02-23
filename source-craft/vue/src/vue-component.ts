import { Context, ContextProvided, FnDefinition, NpmOrLocalImport } from '@source-craft/web-apps';
import { ElementContent } from 'hast';
import ts from 'typescript';

export interface VueComponent {
  template: ElementContent[];
  mounted?: FnDefinition;
  imports?: NpmOrLocalImport[];

  // Property name -> Property Type
  properties?: Record<string, ts.VariableDeclaration>;
  // Field name -> Property Type
  data?: Record<string, ts.VariableDeclaration>;

  provide?: ContextProvided[];
  inject?: Context[];

  methods?: FnDefinition[] | undefined;
  subcomponents?: VueSubcomponent[];
}

export interface VueSubcomponent {
  identifier: string;
  import: ts.ImportDeclaration;
}
