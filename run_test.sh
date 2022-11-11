#!/usr/bin/bash
set -e

rm -rf /tmp/forum-lit
cd /tmp

hc-scaffold web-app forum-lit --setup-nix true --template lit
cd forum-lit
nix-shell . --run "
set -e
npm i 
hc-scaffold dna forum 
hc-scaffold zome posts --integrity dnas/forum/zomes/integrity/ --coordinator dnas/forum/zomes/coordinator/
hc-scaffold entry-type post posts --crud crud --link-from-original-to-each-update true --depends-on --depends-on-itself false --fields
hc-scaffold entry-type comment comments --crud crud --link-from-original-to-each-update false --depends-on post --depends-on-itself false --fields
hc-scaffold entry-type like likes --crud crd --depends-on --depends-on-itself false --fields
hc-scaffold entry-type certificate certificates --crud cr --depends-on post,agent --depends-on-itself false --fields

hc-scaffold index global all_posts --entry-types post --link-to-entry-hash false
hc-scaffold index by-author posts_by_author --entry-types post --link-to-entry-hash false
hc-scaffold index global all_posts_entry_hash --entry-types post --link-to-entry-hash true
hc-scaffold index by-author posts_by_author_entry_hash --entry-types post --link-to-entry-hash true

hc-scaffold link-type post like --link-from-entry-hash false --link-to-entry-hash false
hc-scaffold link-type comment like --link-from-entry-hash false --link-to-entry-hash true
hc-scaffold link-type certificate like --link-from-entry-hash true --link-to-entry-hash false

npm t
npm run package
"