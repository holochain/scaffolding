import { execSync } from 'child_process';
import { chdir } from 'process';
import os from 'os';

const installNixCommands = ['sh <(curl -L -k https://nixos.org/nix/install)', '. ~/.nix-profile/etc/profile.d/nix.sh'];

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
    } else {
      await installNix();
    }

    globalCommands.forEach(execute);

    console.log(`> Automatic setup: cd ${happName}`);

    chdir(happName);
    console.log('');

    localCommands.forEach(execute);

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

function execute(command: string) {
  console.log('> Automatic Setup: ', command);
  console.log('');
  execSync(command, {
    stdio: ['inherit', 'inherit', 'inherit'],
  });
  console.log('');
}

async function installNix() {
  try {
    if (isMacCatalinaOrMore()) {
      execute('sudo mount -uw /');
      execute('sh <(curl -L https://nixos.org/nix/install) --darwin-use-unencrypted-nix-store-volume');
    } else {
      execute('sh <(curl -L -k https://nixos.org/nix/install)');
    }

    execute('. ~/.nix-profile/etc/profile.d/nix.sh');

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

export function isMacCatalinaOrMore() {
  if (os.platform() !== 'darwin') return false;
  let [majorStr, minorStr, _] = os.release().split('.'); //'10.8.0'
  const major = parseInt(majorStr);
  const minor = parseInt(minorStr);
  if (major === 10) return minor >= 15;
  else return major > 10;
}
