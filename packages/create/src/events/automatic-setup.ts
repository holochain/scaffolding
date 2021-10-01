import prompts from 'prompts';
import { execSync } from 'child_process';
import { chdir } from 'process';

const installNixCommand = 'curl -L https://nixos.org/nix/install | sh';

const localCommands = ['nix-shell', 'npm install'];

const globalCommands = [
  installNixCommand,
  '. ~/.nix-profile/etc/profile.d/nix.sh',
  'nix-env -iA cachix -f https://cachix.org/api/v1/install',
  'cachix use holochain-ci',
];

export async function automaticSetup(happName: string) {
  console.log('> Automatic Setup: we are about to execute these commands:');
  console.log('');

  for (const command of [...globalCommands, `cd ${happName}`, ...localCommands]) {
    console.log(command);
  }
  console.log('');

  const response = await prompts({
    type: 'confirm',
    name: 'value',
    message: 'Execute automatic setup?',
    initial: true,
  });

  console.log('');

  if (response) {
    try {
      if (!isNixInstalled()) {
        await installNix();
      }

      executeCommands(globalCommands.slice(1));

      console.log(`> Automatic setup: cd ${happName}`);

      chdir(happName);
      console.log('');

      executeCommands(localCommands);

      console.log('> Automatic setup: setup completed!');
    } catch (e) {
      console.error('> Automatic setup: there was an error executing the automatic setup, exiting...');
      process.exit();
    }
  }
  console.log(`To get started, execute these commands: 

cd ${happName}
nix-shell
npm run build:happ
npm start
`);

  process.exit();
}

function executeCommands(commands: string[]) {
  for (let i = 1; i < commands.length; i++) {
    console.log('> Automatic Setup: ', commands[i]);
    console.log('');
    execSync(commands[i], {
      stdio: ['inherit', 'inherit', 'inherit'],
    });
    console.log('');
  }
}

async function installNix() {
  try {
    execSync(installNixCommand, {
      stdio: ['inherit', 'inherit', 'inherit'],
    });
    if (!isNixInstalled()) {
      throw new Error(
        'Could not install Nix, try to install it manually at https://nixos.org/download.html#nix-quick-install',
      );
    }
  } catch (e) {
    console.error('There was an error installing Nix:', JSON.stringify(e));
  }
}

function isNixInstalled(): boolean {
  try {
    execSync('nix-shell --version', {
      stdio: ['inherit', 'inherit', 'inherit'],
    });
    return true;
  } catch (e) {
    return false;
  }
}
