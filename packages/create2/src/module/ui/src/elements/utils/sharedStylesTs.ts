import { ScFile, ScNodeType } from '@source-craft/types';
import camelCase from 'lodash-es/camelCase';
import kebabCase from 'lodash-es/kebabCase';
import upperFirst from 'lodash-es/upperFirst';
import snakeCase from 'lodash-es/snakeCase';

export const sharedStylesTs = (): ScFile => ({
  type: ScNodeType.File,
  content: `import { css } from 'lit';

export const sharedStyles = css\`
  .row {
    display: flex;
    flex-direction: row;
  }
  .column {
    display: flex;
    flex-direction: column;
  }
  .small-margin {
    margin-top: 6px;
  }
  .big-margin {
    margin-top: 23px;
  }

  .fill {
    flex: 1;
    height: 100%;
  }

  .title {
    font-size: 20px;
  }

  .center-content {
    align-items: center;
    justify-content: center;
  }

  .placeholder {
    color: rgba(0, 0, 0, 0.7);
  }

  .label {
    color: var(--mdc-text-field-label-ink-color, rgba(0, 0, 0, 0.6));
    font-family: var(
      --mdc-typography-caption-font-family,
      var(--mdc-typography-font-family, Roboto, sans-serif)
    );
    font-size: var(--mdc-typography-caption-font-size, 0.79rem);
    font-weight: var(--mdc-typography-caption-font-weight, 400);
  }
\`;
`
});
    