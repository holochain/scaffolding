import { IntegrityZomeDefinition, CoordinatorZomeDefinition } from '@holochain-scaffolding/definitions';


type CoordinatorZomes = CoordinatorZomeDefinition[];

export const coordinatorZomesFromIntegrityZomes = (integrityZomes: IntegrityZomeDefinition[]): CoordinatorZomes => {
  let coordinatorZomes: CoordinatorZomes = [];

  for (const integrityZome of integrityZomes) {
    const coordinatorZome: CoordinatorZomeDefinition = {
      name: integrityZome.name.slice(0, -10), // cutting off the "_integrity" part of the name
      dependencies: [integrityZome]
    }
    coordinatorZomes.push(coordinatorZome);
  }
  return coordinatorZomes;
};

