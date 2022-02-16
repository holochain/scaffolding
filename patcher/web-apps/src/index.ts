import { Element, ElementContent } from 'hast';

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
  onMounted?: {
    callback: FnDefinition;
    async: boolean;
  };
  imports?: string[];

  // Property name -> Property Type
  properties?: Record<string, WebComponentProp>;
  // Field name -> Property Type
  localState?: Record<string, WebComponentProp>;

  provide?: ServiceProvided[];
  inject?: Service[];

  // Fn name -> fn content
  methods?: Record<string, FnDefinition> | undefined;
  subcomponents?: string[];
}

export interface Service {
  name: string;
  type: string;
}

export interface ServiceProvided {
  service: Service;
  imports: string[];
  createFnContent: string;
}

export interface WebComponentProp {
  type: string;
  default?: string | undefined;
}

export interface HTMLNode {
  tag: string;
  attributes?: string[];
  properties?: Record<string, string>;
  events?: Record<string, string>;
  style?: string;

  ifCondition?: string;
  forLoop?: string;

  inner?: Array<HTMLNode | string>;
}

export interface FnDefinition {
  params: FnParam[];
  fnContent: string;
}

export interface FnParam {
  name: string;
  type: string;
}
