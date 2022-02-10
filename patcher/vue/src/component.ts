import { PatcherFile, PatcherNodeType } from '@patcher/types';
import { HTMLNode, WebComponent } from '@patcher/web-apps';

export function vueComponent(component: WebComponent, rawBindings = ''): PatcherFile {
  const template = `<template>
  ${vueTemplate(component.template)}
</template>`;
  const script = `<script lang="ts">
${vueScript(component, rawBindings)}
</script>`;

  return {
    type: PatcherNodeType.File,
    content: `${template}
${script}`,
  };
}

function vueScript(component: WebComponent, rawBindings: string): string {
  return `${component.imports?.join('\n') || ''}
${component.properties ? `import { PropType, defineComponent } from 'vue';` : `import { defineComponent } from 'vue';`}

export default defineComponent${rawBindings ? `<any, ${rawBindings}>` : ''}({ ${vueSubcomponents(component)} ${vueProps(
    component,
  )} ${vueData(component)} ${vueLifecycle(component)} ${vueProvide(component)} ${vueInject(component)}
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
  props() {
    return {
      ${Object.entries(component.properties)
        .map(
          ([propName, prop]) => `${propName}: {
        type: Object as PropType<${prop.type}>,
        ${prop.default ? `default: ${prop.default}` : ''}
      }`,
        )
        .join(',\n      ')}
    }
  },`;
}

function vueData(component: WebComponent): string {
  if (!component.localState || Object.entries(component.localState).length === 0) return '';

  return `
  data(): { ${Object.entries(component.localState)
    .map(([fieldName, localState]) => `${fieldName}: ${localState.type};`)
    .join(' ')} } {
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
        .map(({ service: { name, type } }) => `${name}: computed(() => this.${name})`)
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
    ? `  ${component.onMounted.async ? 'async ' : ''}mounted(${component.onMounted.callback.params}) {
    ${component.onMounted.callback.fnContent}
  },`
    : ''
}`;
}

function vueTemplate(node: HTMLNode, spaces = 2): string {
  const spacesStr = Array(spaces)
    .fill(' ')
    .join('');
  return `<${mainTag(node)}>
${spacesStr}  ${
    node.inner ? node.inner.map(i => (typeof i === 'string' ? i : vueTemplate(i, spaces + 2))).join('\n') : ''
  }
${spacesStr}</${node.tag}>`;
}

function mainTag(node: HTMLNode): string {
  let tagComponents: string[] = [node.tag];

  if (node.ifCondition) tagComponents.push(`v-if="${node.ifCondition}"`);
  if (node.attributes) tagComponents.push(node.attributes.join(' '));
  if (node.properties) tagComponents.push(vueChildProps(node.properties));
  if (node.events) tagComponents.push(vueChildEvents(node.events));
  if (node.style) tagComponents.push(`style="${node.style}"`);

  return tagComponents.join(' ');
}

function vueChildProps(props: Record<string, string>) {
  return Object.entries(props)
    .map(([key, value]) => `:${key}="${value}"`)
    .join(' ');
}

function vueChildEvents(events: Record<string, string>) {
  return Object.entries(events)
    .map(([key, value]) => `@${key}="${value}"`)
    .join(' ');
}
