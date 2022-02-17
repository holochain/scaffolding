import { PatcherFile, PatcherNodeType } from '@patcher/types';
import { HTMLNode, WebComponent } from '@patcher/web-apps';
import { Root, Element, Properties, ElementContent } from 'hast';
import { Plugin } from 'unified';
import { visit } from 'unist-util-visit';
import { unified } from 'unified';
import rehypeStringify from 'rehype-stringify';
import rehypeFormat from 'rehype-format';
import uniq from 'lodash-es/uniq';

export function vueComponent(component: WebComponent): PatcherFile {
  const template = `<template>
${component.template.map(t => vueTemplate(t as Element)).join('\n')}
</template>`;
  const script = `<script lang="ts">
${vueScript(component)}
</script>`;

  return {
    type: PatcherNodeType.File,
    content: `${template}
${script}`,
  };
}

function vueScript(component: WebComponent): string {
  const rawBindings = component.inject && component.inject.map(i => `${i.name}: ${i.type}; `).join('');
  let imports = [];
  if (component.imports) {
    imports = [...imports, ...component.imports];
  }
  if (component.inject) {
    for (const i of component.inject) {
      imports = [...imports, ...i.imports];
    }
  }
  if (component.methods) {
    for (const i of Object.values(component.methods)) {
      imports = [...imports, ...i.imports];
    }
  }
  if (component.onMounted) {
    imports = [...imports, ...component.onMounted.imports];
  }

  const template = rawBindings ? `<${localDataType(component)}, { ${rawBindings}}>` : '';

  return `${uniq(imports).join('\n') || ''}
${component.properties ? `import { PropType, defineComponent } from 'vue';` : `import { defineComponent } from 'vue';`}

export default defineComponent({ ${vueSubcomponents(component)} ${vueProps(component)} ${vueData(
    component,
  )} ${vueLifecycle(component)} ${vueMethods(component)} ${vueProvide(component)} ${vueInject(component)}
})`;
}

function vueSubcomponents(component: WebComponent): string {
  if (!component.subcomponents || Object.entries(component.subcomponents).length === 0) return '';

  return `
  components: {
    ${component.subcomponents.join(',\n    ')}
  },`;
}

function vueProps(component: WebComponent): string {
  if (!component.properties || Object.entries(component.properties).length === 0) return '';

  return `
  props: {
    ${Object.entries(component.properties)
      .map(
        ([propName, prop]) => `${propName}: {
      type: Object as PropType<${prop.type}>,
      ${
        prop.default
          ? `default: ${prop.default}
  `
          : ''
      }}`,
      )
      .join(',\n      ')}
  },`;
}

function localDataType(component: WebComponent): string {
  return `{ ${component.localState &&
    Object.entries(component.localState)
      .map(([fieldName, localState]) => `${fieldName}: ${localState.type};`)
      .join(' ')} }`;
}

function vueData(component: WebComponent): string {
  if (!component.localState || Object.entries(component.localState).length === 0) return '';

  return `
  data(): ${localDataType(component)} {
    return {
      ${Object.entries(component.localState)
        .map(([fieldName, localState]) => `${fieldName}: ${localState.default || 'undefined'}`)
        .join(',\n      ')}
    }
  },`;
}

function vueProvide(component: WebComponent): string {
  if (!component.provide || component.provide.length === 0) return '';

  return `
  provide() {
    return {
      ${component.provide
        .map(({ context: { name, type } }) => `${name}: computed(() => this.${name})`)
        .join(',\n      ')}
    }
  },`;
}

function vueInject(component: WebComponent): string {
  if (!component.inject || component.inject.length === 0) return '';

  return `
  inject: ['${component.inject.map(({ name }) => name)}'],`;
}

function vueLifecycle(component: WebComponent): string {
  return `
${
  component.onMounted
    ? `  ${component.onMounted.async ? 'async ' : ''}mounted(${component.onMounted.params}) {
    ${component.onMounted.fnContent}
  },`
    : ''
}`;
}

function vueMethods(component: WebComponent): string {
  if (!component.methods) return '';

  return `
  methods: {
    ${Object.entries(component.methods)
      .map(
        ([fnName, fn]) => `${fn.async ? 'async ' : ''}${fnName}(${fn.params
          .map(p => `${p.name}: ${p.type}`)
          .join(', ')}) {
      ${fn.fnContent}
    },`,
      )
      .join('\n    ')}
  },`;
}

export const vueTemplatePlugin: Plugin<[], Element> = () => {
  return tree => {
    visit(tree, 'ifCondition', node => {
      let newNode = (node as any) as Element;
      newNode.type = 'element';
      newNode.tagName = 'div';

      newNode.children = [
        {
          type: 'element',
          tagName: 'div',
          properties: {
            'v-if': node.condition,
          },
          children: [node.then],
        },
      ];

      if (node.else) {
        newNode.children.push({
          type: 'element',
          tagName: 'div',
          properties: {
            'v-else': '',
          },
          children: [node.else],
        });
      }
    });

    visit(tree, 'element', node => {
      if (node.inputs) {
        if (!node.properties) node.properties = {};
        for (const [inputName, inputValue] of Object.entries(node.inputs)) {
          node.properties[`:${inputName}`] = inputValue;
        }
      }
      
      if (node.events) {
        if (!node.properties) node.properties = {};
        for (const [eventName, eventValue] of Object.entries(node.events)) {
          node.properties[`@${eventName}`] = eventValue;
        }
      }
    });
    return tree;
  };
};

function vueTemplate(element: Element): string {
  const d = unified()
    .use(vueTemplatePlugin)
    .use(rehypeFormat, {
      indent: '  ',
    })
    .runSync(element);

  return unified()
    .use(rehypeStringify, {
      collapseEmptyAttributes: true
    })
    .stringify(d as any);
}
