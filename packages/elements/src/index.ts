import '@material/mwc-textfield';
import '@material/mwc-textarea';
import '@material/mwc-button';
import '@material/mwc-icon-button';
import '@material/mwc-fab';
import '@material/mwc-list';
import '@material/mwc-dialog';
import '@material/mwc-drawer';
import '@material/mwc-select';
import '@material/mwc-ripple';
import '@material/mwc-checkbox';
import '@material/mwc-formfield';
import { SlBreadcrumb, SlBreadcrumbItem } from '@scoped-elements/shoelace';
customElements.define('sl-breadcrumb', SlBreadcrumb);
customElements.define('sl-breadcrumb-item', SlBreadcrumbItem);

import { Card } from '@scoped-elements/material-web';
customElements.define('mwc-card', Card as any);

import { defineCustomElement } from 'vue';

import HappDefinitionBuilderVue from './elements/HappDefinitionBuilder.ce.vue';

const HappDefinitionBuilder = defineCustomElement(HappDefinitionBuilderVue);

export { HappDefinitionBuilder };
export * from './utils';
