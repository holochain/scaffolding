import { ScFile, ScNodeType } from '@source-craft/types';
import camelCase from 'lodash-es/camelCase';
import kebabCase from 'lodash-es/kebabCase';
import upperFirst from 'lodash-es/upperFirst';
import snakeCase from 'lodash-es/snakeCase';

export const indexTs = ({testZomeName, moduleNameSnakeCase, moduleNamePluralTitleCase, kebabPlural_, moduleNameTitleCase, moduleNamePlural, moduleName}: {testZomeName: string; moduleNameSnakeCase: string; moduleNamePluralTitleCase: string; kebabPlural_: string; moduleNameTitleCase: string; moduleNamePlural: string; moduleName: string;}): ScFile => ({
  type: ScNodeType.File,
  content: `import { Config, InstallAgentsHapps, Orchestrator } from "@holochain/tryorama";
import Base64 from "js-base64";
import path from "path";

const conductorConfig = Config.gen();

// Construct proper paths for your DNAs
const ${camelCase(moduleNamePlural)}Dna = path.join(__dirname, "../../workdir/dna/${kebabPlural_}test.dna");

// create an InstallAgentsHapps array with your DNAs to tell tryorama what
// to install into the conductor.
const installation: InstallAgentsHapps = [
  // agent 0
  [
    // happ 0
    [${camelCase(moduleNamePlural)}Dna],
  ],
  [
    // happ 0
    [${camelCase(moduleNamePlural)}Dna],
  ],
];

const sleep = (ms) =>
  new Promise((resolve) => setTimeout(() => resolve(null), ms));

function serializeHash(hash) {
  return \`u\${Base64.fromUint8Array(hash, true)}\`;
}

${testZomeName}

let orchestrator = new Orchestrator();

orchestrator.registerScenario("create a ${camelCase(moduleName)} and get it", async (s, t) => {
  const [alice, bob] = await s.players([conductorConfig]);

  // install your happs into the coductors and destructuring the returned happ data using the same
  // array structure as you created in your installation array.
  const [[alice${moduleNameSnakeCase}s], [bob${moduleNameSnakeCase}s]] = await alice.installAgentsHapps(
    installation
  );


  let alicePubkeyB64 = serializeHash(alice${moduleNameSnakeCase}s.agent);
  let bobPubKeyB64 = serializeHash(bob${moduleNameSnakeCase}s.agent);

  let my${moduleNameTitleCase} = await alice${moduleNameSnakeCase}s.cells[0].call(
    zomeName,
    "get_my${moduleNameSnakeCase}",
    null
  );
  t.notOk(my${moduleNameTitleCase});

  let ${camelCase(moduleName)}Hash = await alice${moduleNameSnakeCase}s.cells[0].call(
    zomeName,
    "create${moduleNameSnakeCase}",
    {
      nickname: "alice",
      fields: {
        avatar: "aliceavatar",
      },
    }
  );
  t.ok(${camelCase(moduleName)}Hash);

  await sleep(500);

  // set nickname as alice to make sure bob's is not getting deleted
  // with alice's update
  ${camelCase(moduleName)}Hash = await bob${moduleNameSnakeCase}s.cells[0].call(zomeName, "create${moduleNameSnakeCase}", {
    nickname: "alice_bob",
    fields: {
      avatar: "bobboavatar",
    },
  });
  t.ok(${camelCase(moduleName)}Hash);

  await sleep(5000);

  ${camelCase(moduleName)}Hash = await alice${moduleNameSnakeCase}s.cells[0].call(
    zomeName,
    "update${moduleNameSnakeCase}",
    {
      nickname: "alice2",
      fields: {
        avatar: "aliceavatar2",
        update: "somenewfield",
      },
    }
  );
  t.ok(${camelCase(moduleName)}Hash);

  my${moduleNameTitleCase} = await alice${moduleNameSnakeCase}s.cells[0].call(
    zomeName,
    "get_my${moduleNameSnakeCase}",
    null
  );
  t.ok(my${moduleNameTitleCase}.agentPubKey);
  t.equal(my${moduleNameTitleCase}.${camelCase(moduleName)}.nickname, "alice2");

  let all${camelCase(moduleNamePlural)} = await bob${moduleNameSnakeCase}s.cells[0].call(
    zomeName,
    "get_all${moduleNameSnakeCase}s",
    null
  );
  t.equal(all${camelCase(moduleNamePlural)}.length, 2);

  let multiple${moduleNamePluralTitleCase} = await bob${moduleNameSnakeCase}s.cells[0].call(
    zomeName,
    "get_agents${moduleNameSnakeCase}",
    [alicePubkeyB64, bobPubKeyB64]
  );
  t.equal(multiple${moduleNamePluralTitleCase}.length, 2);

  let ${camelCase(moduleNamePlural)} = await bob${moduleNameSnakeCase}s.cells[0].call(
    zomeName,
    "search${moduleNameSnakeCase}s",
    {
      nicknamePrefix: "sdf",
    }
  );
  t.equal(${camelCase(moduleNamePlural)}.length, 0);

  ${camelCase(moduleNamePlural)} = await bob${moduleNameSnakeCase}s.cells[0].call(zomeName, "search${moduleNameSnakeCase}s", {
    nicknamePrefix: "alic",
  });
  t.equal(${camelCase(moduleNamePlural)}.length, 2);
  t.ok(${camelCase(moduleNamePlural)}[0].agentPubKey);
  t.equal(${camelCase(moduleNamePlural)}[1].${camelCase(moduleName)}.nickname, "alice2");

  ${camelCase(moduleNamePlural)} = await bob${moduleNameSnakeCase}s.cells[0].call(zomeName, "search${moduleNameSnakeCase}s", {
    nicknamePrefix: "ali",
  });
  t.equal(${camelCase(moduleNamePlural)}.length, 2);
  t.ok(${camelCase(moduleNamePlural)}[0].agentPubKey);
  t.equal(${camelCase(moduleNamePlural)}[1].${camelCase(moduleName)}.nickname, "alice2");
  t.equal(${camelCase(moduleNamePlural)}[1].${camelCase(moduleName)}.fields.avatar, "aliceavatar2");

  ${camelCase(moduleNamePlural)} = await bob${moduleNameSnakeCase}s.cells[0].call(zomeName, "search${moduleNameSnakeCase}s", {
    nicknamePrefix: "alice",
  });
  t.equal(${camelCase(moduleNamePlural)}.length, 2);
  t.ok(${camelCase(moduleNamePlural)}[1].agentPubKey);
  t.equal(${camelCase(moduleNamePlural)}[1].${camelCase(moduleName)}.nickname, "alice2");

  ${camelCase(moduleNamePlural)} = await bob${moduleNameSnakeCase}s.cells[0].call(zomeName, "search${moduleNameSnakeCase}s", {
    nicknamePrefix: "alice_",
  });
  t.equal(${camelCase(moduleNamePlural)}.length, 2);
  t.ok(${camelCase(moduleNamePlural)}[0].agentPubKey);
  t.equal(${camelCase(moduleNamePlural)}[0].${camelCase(moduleName)}.nickname, "alice_bob");
  t.equal(${camelCase(moduleNamePlural)}[0].${camelCase(moduleName)}.fields.avatar, "bobboavatar");
});

orchestrator.run();
orchestrator = new Orchestrator();

orchestrator.registerScenario(
  "create a ${camelCase(moduleName)} with upper case and search it with lower case",
  async (s, t) => {
    const [alice, bob] = await s.players([conductorConfig]);

    // install your happs into the coductors and destructuring the returned happ data using the same
    // array structure as you created in your installation array.
    const [[alice${moduleNameSnakeCase}s], [bob${moduleNameSnakeCase}s]] = await alice.installAgentsHapps(
      installation
    );

    let ${camelCase(moduleName)}Hash = await alice${moduleNameSnakeCase}s.cells[0].call(
      zomeName,
      "create${moduleNameSnakeCase}",
      {
        nickname: "ALIce",
        fields: {
          avatar: "aliceavatar",
        },
      }
    );
    t.ok(${camelCase(moduleName)}Hash);
    await sleep(5000);

    let ${camelCase(moduleNamePlural)} = await bob${moduleNameSnakeCase}s.cells[0].call(
      zomeName,
      "search${moduleNameSnakeCase}s",
      {
        nicknamePrefix: "ali",
      }
    );
    t.equal(${camelCase(moduleNamePlural)}.length, 1);
    t.ok(${camelCase(moduleNamePlural)}[0].agentPubKey);
    t.equal(${camelCase(moduleNamePlural)}[0].${camelCase(moduleName)}.nickname, "ALIce");

    ${camelCase(moduleNamePlural)} = await bob${moduleNameSnakeCase}s.cells[0].call(zomeName, "search${moduleNameSnakeCase}s", {
      nicknamePrefix: "aLI",
    });
    t.equal(${camelCase(moduleNamePlural)}.length, 1);
    t.ok(${camelCase(moduleNamePlural)}[0].agentPubKey);
    t.equal(${camelCase(moduleNamePlural)}[0].${camelCase(moduleName)}.nickname, "ALIce");

    ${camelCase(moduleNamePlural)} = await bob${moduleNameSnakeCase}s.cells[0].call(zomeName, "search${moduleNameSnakeCase}s", {
      nicknamePrefix: "AlI",
    });
    t.equal(${camelCase(moduleNamePlural)}.length, 1);
    t.ok(${camelCase(moduleNamePlural)}[0].agentPubKey);
    t.equal(${camelCase(moduleNamePlural)}[0].${camelCase(moduleName)}.nickname, "ALIce");

    ${camelCase(moduleNamePlural)} = await bob${moduleNameSnakeCase}s.cells[0].call(zomeName, "search${moduleNameSnakeCase}s", {
      nicknamePrefix: "ALI",
    });
    t.equal(${camelCase(moduleNamePlural)}.length, 1);
    t.ok(${camelCase(moduleNamePlural)}[0].agentPubKey);
    t.equal(${camelCase(moduleNamePlural)}[0].${camelCase(moduleName)}.nickname, "ALIce");
  }
);

orchestrator.run();
`
});
    