import { ReactiveController, ReactiveElement } from 'lit';

export class UniqueFieldsController implements ReactiveController {
  currentFields: Array<HTMLInputElement> = [];

  constructor(
    protected host: ReactiveElement,
    protected getFields: () => HTMLInputElement[],
    protected duplicatedMessage = 'Must be unique'
  ) {
    this.host.addController(this);
  }

  checkDuplicates = () => {
    for (const f of this.currentFields) {
      f.setCustomValidity('');
      f.reportValidity();
    }

    for (let i = 0; i < this.currentFields.length; i++) {
      for (let j = 0; j < this.currentFields.length; j++) {
        if (i != j) {
          if (this.currentFields[i].value === this.currentFields[j].value) {
            this.currentFields[i].setCustomValidity(this.duplicatedMessage);
            this.currentFields[j].setCustomValidity(this.duplicatedMessage);
            this.currentFields[i].reportValidity();
            this.currentFields[j].reportValidity();
          }
        }
      }
    }
  };

  hostUpdated() {
    const fields = this.getFields();

    const fieldsToAdd = fields.filter(
      f => !this.currentFields.find(oldField => f === oldField)
    );
    const fieldsToRemove = this.currentFields.filter(
      f => !fields.find(newField => f === newField)
    );

    for (const toRemove of fieldsToRemove) {
      toRemove.removeEventListener('input', this.checkDuplicates);
    }
    for (const toAdd of fieldsToAdd) {
      toAdd.addEventListener('input', this.checkDuplicates);
    }

    this.currentFields = fields;

    setTimeout(()=> {

      this.checkDuplicates();
    })
  }
}
