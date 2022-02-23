import { Element, ElementContent } from 'hast';
import { NpmImport } from '@source-craft/npm';
import * as ts from 'typescript';

export type NpmOrLocalImport = NpmImport | ts.ImportDeclaration;

export interface IfCondition {
  type: 'ifCondition';
  condition: string;
  then: Element;
  else?: Element;
}

declare module 'hast' {
  interface ElementContentMap {
    ifCondition: IfCondition;
  }

  interface Element {
    inputs?: Properties | undefined;
    events?: Record<string, string> | undefined;
  }
}

export interface WebComponent {
  template: ElementContent[];
  onMounted?: FnDefinition;
  imports?: NpmOrLocalImport[];

  // Property name -> Property Type
  properties?: Record<string, WebComponentProp>;
  // Field name -> Property Type
  localState?: Record<string, WebComponentProp>;

  provide?: ContextProvided[];
  inject?: Context[];

  // Fn name -> fn content
  methods?: FnDefinition[] | undefined;
  subcomponents?: string[];
}

export interface Context {
  name: string;
  type: string;
  imports: NpmOrLocalImport[];
}

export interface ContextProvided {
  context: Context;
  createContext: FnDefinition;
}

export interface WebComponentProp {
  type: string;
  default?: string | undefined;
}

export interface FnDefinition {
  imports: NpmOrLocalImport[];
  declaration: ts.FunctionDeclaration;
}
