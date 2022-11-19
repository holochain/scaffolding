#!/usr/bin/bash
set -e

# rm -rf /tmp/forum-vue
# cd /tmp

# hc-scaffold web-app forum-vue --setup-nix false --template vue
# cd forum-vue
# hc-scaffold dna forum 
# hc-scaffold zome posts --integrity dnas/forum/zomes/integrity/ --coordinator dnas/forum/zomes/coordinator/
# hc-scaffold entry-type post --fixed false --crud crud --link-from-original-to-each-update true --depends-on --depends-on-itself false --fields
# hc-scaffold entry-type comment --fixed false --crud crud --link-from-original-to-each-update false --depends-on post --depends-on-itself false --fields
# hc-scaffold entry-type like --fixed false --crud crd --depends-on --depends-on-itself option --fields
# hc-scaffold entry-type certificate --fixed true --crud cr --depends-on post,agent:certified --depends-on-itself vector --fields

# hc-scaffold index global all_posts post 
# hc-scaffold index by-author posts_by_author post
# hc-scaffold index global all_posts_entry_hash post:EntryHash
# hc-scaffold index by-author posts_by_author_entry_hash post:EntryHash

# hc-scaffold link-type post like --bidireccional false
# hc-scaffold link-type comment like:EntryHash --bidireccional true
# hc-scaffold link-type certificate:EntryHash like --bidireccional false
# hc-scaffold link-type agent:Creator post:EntryHash --bidireccional true

# nix-shell https://holochain.love --run "
# set -e
# npm i
# npm run build -w ui
# "

# rm -rf /tmp/forum-lit
# cd /tmp

# hc-scaffold web-app forum-lit --setup-nix false --template lit
# cd forum-lit
# hc-scaffold dna forum 
# hc-scaffold zome posts --integrity dnas/forum/zomes/integrity/ --coordinator dnas/forum/zomes/coordinator/
# hc-scaffold entry-type post --fixed false --crud crud --link-from-original-to-each-update true --depends-on --depends-on-itself false --fields
# hc-scaffold entry-type comment --fixed false --crud crud --link-from-original-to-each-update false --depends-on post --depends-on-itself false --fields
# hc-scaffold entry-type like --fixed false --crud crd --depends-on --depends-on-itself option --fields
# hc-scaffold entry-type certificate --fixed true --crud cr --depends-on post,agent:certified --depends-on-itself vector --fields

# hc-scaffold index global all_posts post 
# hc-scaffold index by-author posts_by_author post
# hc-scaffold index global all_posts_entry_hash post:EntryHash
# hc-scaffold index by-author posts_by_author_entry_hash post:EntryHash

# hc-scaffold link-type post like --bidireccional false
# hc-scaffold link-type comment like:EntryHash --bidireccional true
# hc-scaffold link-type certificate:EntryHash like --bidireccional false
# hc-scaffold link-type agent:Creator post:EntryHash --bidireccional true

# nix-shell https://holochain.love --run "
# set -e
# npm i
# npm run build -w ui
# npm run format -w ui
# npm run lint -w ui
# "


rm -rf /tmp/forum-vanilla
cd /tmp

hc-scaffold web-app forum-vanilla --setup-nix true --template vanilla
cd forum-vanilla
hc-scaffold dna forum 
hc-scaffold zome posts --integrity dnas/forum/zomes/integrity/ --coordinator dnas/forum/zomes/coordinator/
hc-scaffold entry-type post --fixed false --crud crud --link-from-original-to-each-update true --depends-on --depends-on-itself false --fields
hc-scaffold entry-type comment --fixed false --crud crud --link-from-original-to-each-update false --depends-on post --depends-on-itself false --fields
hc-scaffold entry-type like --fixed false --crud crd --depends-on --depends-on-itself option --fields
hc-scaffold entry-type certificate --fixed true --crud cr --depends-on post,agent:certified --depends-on-itself vector --fields

hc-scaffold index global all_posts post 
hc-scaffold index by-author posts_by_author post
hc-scaffold index global all_posts_entry_hash post:EntryHash
hc-scaffold index by-author posts_by_author_entry_hash post:EntryHash

hc-scaffold link-type post like --bidireccional false
hc-scaffold link-type comment like:EntryHash --bidireccional true
hc-scaffold link-type certificate:EntryHash like --bidireccional false
hc-scaffold link-type agent:Creator post:EntryHash --bidireccional true

nix-shell . --run "
set -e
npm i
npm t
npm run package
"