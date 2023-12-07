#!/usr/bin/env bash
set -e

rm -rf /tmp/forum-svelte

cd /tmp

echo $(hc-scaffold --version)

hc-scaffold web-app forum-svelte --setup-nix true --template svelte

cd forum-svelte

hc-scaffold dna forum 
hc-scaffold zome posts --integrity dnas/forum/zomes/integrity/ --coordinator dnas/forum/zomes/coordinator/
hc-scaffold entry-type post --reference-entry-hash false --crud crud --link-from-original-to-each-update true --fields title:String:TextField,content:String:TextArea
hc-scaffold entry-type comment --reference-entry-hash false --crud crud --link-from-original-to-each-update false --fields post_hash:ActionHash::Post
hc-scaffold entry-type like --reference-entry-hash false --crud crd --fields like_hash:Option\<ActionHash\>::Like,string_list:Vec\<String\>
hc-scaffold entry-type certificate --reference-entry-hash true --crud cr --fields post_hash:ActionHash::Post,agent:AgentPubKey::certified,certifications_hashes:Vec\<EntryHash\>::Certificate,certificate_type:Enum::CertificateType:TypeOne.TypeTwo,dna_hash:DnaHash

hc-scaffold collection global all_posts post 
hc-scaffold collection by-author posts_by_author post
hc-scaffold collection global all_posts_entry_hash post:EntryHash
hc-scaffold collection global all_likes like
hc-scaffold collection by-author posts_by_author_entry_hash post:EntryHash

hc-scaffold link-type post like --delete true --bidireccional false
hc-scaffold link-type comment like:EntryHash --delete true --bidireccional true
hc-scaffold link-type certificate:EntryHash like --delete false --bidireccional false
hc-scaffold link-type agent:creator post:EntryHash --delete false --bidireccional true


nix develop --command bash -c "
set -e
npm i
npm run build -w ui
npm t
npm run package
"

rm -rf /tmp/forum-vue
cd /tmp

hc-scaffold web-app forum-vue --setup-nix true --template vue
cd forum-vue

hc-scaffold dna forum 
hc-scaffold zome posts --integrity dnas/forum/zomes/integrity/ --coordinator dnas/forum/zomes/coordinator/
hc-scaffold entry-type post --reference-entry-hash false --crud crud --link-from-original-to-each-update true --fields title:String:TextField,content:String:TextArea
hc-scaffold entry-type comment --reference-entry-hash false --crud crud --link-from-original-to-each-update false --fields post_hash:ActionHash::Post
hc-scaffold entry-type like --reference-entry-hash false --crud crd --fields like_hash:Option\<ActionHash\>::Like,string_list:Vec\<String\>
hc-scaffold entry-type certificate --reference-entry-hash true --crud cr --fields post_hash:ActionHash::Post,agent:AgentPubKey::certified,certifications_hashes:Vec\<EntryHash\>::Certificate,certificate_type:Enum::CertificateType:TypeOne.TypeTwo,dna_hash:DnaHash

hc-scaffold collection global all_posts post 
hc-scaffold collection by-author posts_by_author post
hc-scaffold collection global all_likes like
hc-scaffold collection global all_posts_entry_hash post:EntryHash
hc-scaffold collection by-author posts_by_author_entry_hash post:EntryHash

hc-scaffold link-type post like --delete true --bidireccional false
hc-scaffold link-type comment like:EntryHash --delete true --bidireccional true
hc-scaffold link-type certificate:EntryHash like --delete false --bidireccional false
hc-scaffold link-type agent:creator post:EntryHash --delete false --bidireccional true

nix develop --command bash -c "
set -e
npm i
npm run build -w ui
npm t
npm run package
"

rm -rf /tmp/forum-lit
cd /tmp

hc-scaffold web-app forum-lit --setup-nix true --template lit
cd forum-lit

hc-scaffold dna forum 
hc-scaffold zome posts --integrity dnas/forum/zomes/integrity/ --coordinator dnas/forum/zomes/coordinator/
hc-scaffold entry-type post --reference-entry-hash false --crud crud --link-from-original-to-each-update true --fields title:String:TextField,content:String:TextArea
hc-scaffold entry-type comment --reference-entry-hash false --crud crud --link-from-original-to-each-update false --fields post_hash:ActionHash::Post
hc-scaffold entry-type like --reference-entry-hash false --crud crd --fields like_hash:Option\<ActionHash\>::Like,string_list:Vec\<String\>
hc-scaffold entry-type certificate --reference-entry-hash true --crud cr --fields post_hash:ActionHash::Post,agent:AgentPubKey::certified,certifications_hashes:Vec\<EntryHash\>::Certificate,certificate_type:Enum::CertificateType:TypeOne.TypeTwo,dna_hash:DnaHash

hc-scaffold collection global all_posts post 
hc-scaffold collection by-author posts_by_author post
hc-scaffold collection global all_posts_entry_hash post:EntryHash
hc-scaffold collection by-author posts_by_author_entry_hash post:EntryHash
hc-scaffold collection global all_likes like

hc-scaffold link-type post like --delete true --bidireccional false
hc-scaffold link-type comment like:EntryHash --delete true --bidireccional true
hc-scaffold link-type certificate:EntryHash like --delete false --bidireccional false
hc-scaffold link-type agent:creator post:EntryHash --delete false --bidireccional true

nix develop --command bash -c "
set -e
npm i
npm run build -w ui
npm run format -w ui
npm run lint -w ui
npm t
npm run package
"


rm -rf /tmp/forum-vanilla
cd /tmp

hc-scaffold web-app forum-vanilla --setup-nix true --template vanilla
cd forum-vanilla

hc-scaffold dna forum 
hc-scaffold zome posts --integrity dnas/forum/zomes/integrity/ --coordinator dnas/forum/zomes/coordinator/
hc-scaffold entry-type post --reference-entry-hash false --crud crud --link-from-original-to-each-update true --fields title:String:TextField,content:String:TextArea
hc-scaffold entry-type comment --reference-entry-hash false --crud crud --link-from-original-to-each-update false --fields post_hash:ActionHash::Post
hc-scaffold entry-type like --reference-entry-hash false --crud crd --fields like_hash:Option\<ActionHash\>::Like,string_list:Vec\<String\>
hc-scaffold entry-type certificate --reference-entry-hash true --crud cr --fields post_hash:ActionHash::Post,agent:AgentPubKey::certified,certifications_hashes:Vec\<EntryHash\>::Certificate,certificate_type:Enum::CertificateType:TypeOne.TypeTwo,dna_hash:DnaHash

hc-scaffold collection global all_posts post 
hc-scaffold collection by-author posts_by_author post
hc-scaffold collection global all_posts_entry_hash post:EntryHash
hc-scaffold collection global all_likes like
hc-scaffold collection by-author posts_by_author_entry_hash post:EntryHash

hc-scaffold link-type post like --delete true --bidireccional false
hc-scaffold link-type comment like:EntryHash --delete true --bidireccional true
hc-scaffold link-type certificate:EntryHash like --delete false --bidireccional false
hc-scaffold link-type agent:creator post:EntryHash --delete false --bidireccional true


nix develop --command bash -c "
set -e
npm i
npm t
npm run package
"

rm -rf /tmp/hello-world
cd /tmp

hc-scaffold example hello-world
cd hello-world

nix develop --command bash -c "
set -e
npm i
npm t 
"

rm -rf /tmp/holo-flake
cd /tmp

hc-scaffold web-app holo-flake --setup-nix true --template vue --holo
cd holo-flake

nix develop --command bash -c "
set -e

# Check if holo-dev-server is in the path
if command -v holo-dev-server >/dev/null 2>&1; then
    echo 'holo-dev-server is available in the PATH'
else
    echo 'holo-dev-server is NOT available in the PATH'
    exit 1
fi
"