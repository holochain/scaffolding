import { execSync } from 'child_process';
import { chdir } from 'process';

const installNixCommands = ['curl -L -k https://nixos.org/nix/install | sh', '. ~/.nix-profile/etc/profile.d/nix.sh'];

const localCommands = ['nix-shell . --run "npm install"'];

const globalCommands = ['nix-env -iA cachix -f https://cachix.org/api/v1/install', 'cachix use holochain-ci'];

export async function automaticSetup(happName: string) {
  console.log('> Automatic Setup: we are about to execute these commands:');
  console.log('');

  for (const command of [...installNixCommands, ...globalCommands, `cd ${happName}`, ...localCommands]) {
    console.log(command);
  }

  console.log('');

  try {
    if (isNixInstalled()) {
      console.log(`> Automatic setup: nix is already installed, skipping`);
    }else{
      await installNix();
    }

    executeCommands(globalCommands);

    console.log(`> Automatic setup: cd ${happName}`);

    chdir(happName);
    console.log('');

    executeCommands(localCommands);

    console.log('> Automatic setup: setup completed!');
    console.log('');
  } catch (e) {
    console.error('> Automatic setup: there was an error executing the automatic setup, exiting...');
    process.exit();
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
  for (let i = 0; i < commands.length; i++) {
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
    for (const command of installNixCommands) {
      execSync(command, {
        stdio: ['inherit', 'inherit', 'inherit'],
      });
    }
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
