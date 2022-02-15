import { TypeDefinition } from '@typecraft/type-definition';
import { dateType } from '@typecraft/date';

export const defaultTypes: TypeDefinition<any, any>[] = [
  /*   {
    name: 'Rating',
    configuration: {
      properties: {
        minRating: {
          description: 'Minimum rating allowed',
          type: 'number',
          default: 0,
          minimum: 0,
        },
        maxRating: {
          description: 'Maximum rating allowed',
          default: 5,
          type: 'number',
        },
      },
    },
    
  }, */
  dateType,
];
