import { ScFile, ScNodeType } from '@source-craft/types';
import camelCase from 'lodash-es/camelCase';
import kebabCase from 'lodash-es/kebabCase';
import upperFirst from 'lodash-es/upperFirst';
import snakeCase from 'lodash-es/snakeCase';

export const elementsTestJs = ({_kebab, moduleNameTitleCase, moduleName}: {_kebab: string; moduleNameTitleCase: string; moduleName: string;}): ScFile => ({
  type: ScNodeType.File,
  content: `import { html, fixture, expect } from '@open-wc/testing';
import { setupApolloClientMock } from './mocks';
import { HodCreate${moduleNameTitleCase}Form } from '../dist';
import { setupApolloClientElement } from '@holochain-open-dev/common';

describe('HodCreate${moduleNameTitleCase}Form', () => {
  it('create ${moduleName} has a placeholder', async () => {
    const client = await setupApolloClientMock();

    customElements.define(
      'hod-create${_kebab}-form',
      setupApolloClientElement(HodCreate${moduleNameTitleCase}Form, client)
    );

    const el = await fixture(
      html\` <hod-create${_kebab}-form></hod-create${_kebab}-form> \`
    );

    expect(el.shadowRoot.innerHTML).to.include('CREATE PROFILE');
  });
});
`
});
    