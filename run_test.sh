#!/usr/bin/bash

hc-scaffold web-app forum --setup-nix true
cd forum
nix-shell . --run "npm i && hc-scaffold dna forum && hc-scaffold zome posts --path && hc-scaffold entry-type post && npm t"

