import { ScFile, ScNodeType } from '@source-craft/types';
import camelCase from 'lodash-es/camelCase';
import kebabCase from 'lodash-es/kebabCase';
import upperFirst from 'lodash-es/upperFirst';
import snakeCase from 'lodash-es/snakeCase';

export const holoIdenticonTs = (): ScFile => ({
  type: ScNodeType.File,
  content: `import { deserializeHash, HoloHashB64 } from '@holochain-open-dev/core-types';
import { css, html, LitElement, PropertyValues } from 'lit';
import { property, query } from 'lit/decorators.js';
import renderIcon from '@holo-host/identicon';
import { classMap } from 'lit/directives/class-map.js';

export class HoloIdenticon extends LitElement {
  /**
   * REQUIRED. The hash that will be converted to an identicon.
   */
  @property({ type: String })
  hash!: HoloHashB64;

  /**
   * Size of the identicon in pixels.
   */
  @property({ type: Number })
  size = 32;

  /**
   * Shape of the identicon.
   */
  @property({ type: String })
  shape: 'square' | 'circle' = 'circle';

  @query('#canvas')
  private _canvas!: HTMLCanvasElement;

  updated(changedValues: PropertyValues) {
    super.updated(changedValues);

    if (changedValues.has('hash') || changedValues.has('size')) {
      renderIcon(
        {
          hash: deserializeHash(this.hash),
          size: this.size,
        },
        this._canvas
      );
    }
  }

  render() {
    return html\`
      <canvas
        id="canvas"
        width="1"
        height="1"
        class=\${classMap({
          square: this.shape === 'square',
          circle: this.shape === 'circle',
        })}
      ></canvas>
    \`;
  }

  static get styles() {
    return css\`
      .square {
        border-radius: 0%;
      }

      .circle {
        border-radius: 50%;
      }
    \`;
  }
}
`
});
    