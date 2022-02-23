import { TypeDefinition } from '@type-craft/vocabulary';
import { dateType } from '@type-craft/date';

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
