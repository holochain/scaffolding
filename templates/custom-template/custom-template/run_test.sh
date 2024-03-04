#!/usr/bin/bash
set -e

DIR=$(pwd)

nix shell .#hc-scaffold-custom-template --command bash -c "

cd /tmp
rm -rf custom-template-app

hc-scaffold web-app custom-template-app --setup-nix true 
"

cd /tmp/custom-template-app

nix develop --override-input scaffolding "path:$DIR" --command bash -c "

set -e
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

hc-scaffold link-type post like --delete true --bidirectional false
hc-scaffold link-type comment like:EntryHash --delete true --bidirectional true
hc-scaffold link-type certificate:EntryHash like --delete false --bidirectional false
hc-scaffold link-type agent:creator post:EntryHash --delete false --bidirectional true

npm i
npm run build -w ui
npm t
npm run package
"
