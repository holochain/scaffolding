#!/usr/bin/bash
rm -rf /tmp/forum
cd /tmp

hc-scaffold web-app forum --setup-nix true --template lit
cd forum
nix-shell . --run "
npm i 
hc-scaffold dna forum 
hc-scaffold zome posts --integrity dnas/forum/zomes/integrity/ --coordinator dnas/forum/zomes/coordinator/
hc-scaffold entry-type post posts --crud crud --depends-on --depends-on-itself false --fields
npm t
"