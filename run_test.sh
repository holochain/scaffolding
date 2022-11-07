#!/usr/bin/bash
rm -rf /tmp/forum
cd /tmp

hc-scaffold web-app forum --setup-nix true --template lit
cd forum
nix-shell . --run "
hc-scaffold dna forum 
hc-scaffold zome posts --integrity dnas/forum/zomes/integrity/ --coordinator dnas/forum/zomes/coordinator/

npm i 
npm t
"
# hc-scaffold entry-type post

