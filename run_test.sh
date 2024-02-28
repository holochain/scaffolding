#!/usr/bin/env bash
set -e

TEMPLATE_PATH="/tmp"

APP_NAME=
TEMPLATE_NAME=
SCOPE=

# parse args
while getopts ":a:t:s:" opt; do
  case $opt in
    a) APP_NAME="$OPTARG";;
    t) TEMPLATE_NAME="$OPTARG";;
    s) SCOPE="$OPTARG";;
    \?) echo "Invalid option: -$OPTARG" >&2
        exit 1;;
    :)  echo "Option -$OPTARG requires an argument." >&2
        exit 1;;
  esac
done

cleanup_tmp() {
	rm -rf $TEMPLATE_PATH/$1
}

print_version() {
	echo $(hc-scaffold --version)
}

setup_and_build_happ() {
	print_version
	cleanup_tmp $1

	cd $TEMPLATE_PATH
	hc-scaffold --template $2 web-app $1 --setup-nix true 
	cd $1

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

	nix develop --command bash -c "
    set -e
    npm i
    npm run build -w ui
    npm t
    npm run package
    "
	cd ..
}

setup_and_build_hello_world() {
	print_version
	cleanup_tmp hello-world

	cd $TEMPLATE_PATH
	hc-scaffold example hello-world
	cd hello-world

	nix develop --command bash -c "
    set -e
    npm i
    npm t 
    "
	cd ..
}

if [[ -n "$SCOPE" && "$SCOPE" == "hello_world" ]]; then
	setup_and_build_hello_world
	exit 0 # Exit early
fi

if [[ -z "$APP_NAME" || -z "$TEMPLATE_NAME" ]]; then
	echo "Error: APP_NAME and TEMPLATE_NAME environment variables must be set."
	exit 1
fi

case "$TEMPLATE_NAME" in
"svelte" | "lit" | "vue" | "vanilla")
	# Valid template name, proceed
	;;
*)
	echo "Error: TEMPLATE_NAME must be one of 'svelte', 'lit', 'vue', or 'vanilla'."
	exit 1
	;;
esac

cleanup_tmp "$APP_NAME"
setup_and_build_happ "$APP_NAME" "$TEMPLATE_NAME"
