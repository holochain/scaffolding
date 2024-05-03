#!/usr/bin/env bash
set -e

TEMPLATE_PATH="/tmp"

APP_NAME="forum"
TEMPLATE_NAME=
SCOPE=
OVERRIDE_HOLOCHAIN_VERSION=

# parse args
while getopts ":t:s:o:" opt; do
  case $opt in
    t) TEMPLATE_NAME="$OPTARG" ;;
    s) SCOPE="$OPTARG" ;;
    o) OVERRIDE_HOLOCHAIN_VERSION="$OPTARG" ;;
    \?)
      echo "Invalid option: -$OPTARG" >&2
      exit 1
      ;;
    :)
      if [ "$OPTARG" == "o" ]; then
        OVERRIDE_HOLOCHAIN_VERSION=
        continue
      fi
      echo "Option -$OPTARG requires an argument." >&2
      exit 1
      ;;
  esac
done

cleanup_tmp() {
  rm -rf "${TEMPLATE_PATH:?}/$1"
}

print_version() {
  echo "$(hc-scaffold --version)"
}

build_test_with_overriden_holochain_version() {
   nix develop --override-input "versions/holochain" github:holochain/holochain/$OVERRIDE_HOLOCHAIN_VERSION --command bash -c $1
}

setup_and_build_happ() {
  print_version
  cleanup_tmp "$1"

  cd $TEMPLATE_PATH
  hc-scaffold web-app "$1" --setup-nix true --template "$2"
  cd "$1"

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

  hc-scaffold link-type post like --delete true --bidirectional false
  hc-scaffold link-type comment like:EntryHash --delete true --bidirectional true
  hc-scaffold link-type certificate:EntryHash like --delete false --bidirectional false
  hc-scaffold link-type agent:creator post:EntryHash --delete false --bidirectional true

  if [[ -n "$OVERRIDE_HOLOCHAIN_VERSION" ]]; then
    build_test_with_overriden_holochain_version "
      set -e
      npm i
      npm run build -w ui
      npm t
      npm run package
    "
    cd ..
    exit 0
  fi

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

  if [[ -n "$OVERRIDE_HOLOCHAIN_VERSION" ]]; then
    build_test_with_overriden_holochain_version "
      set -e
      npm i
      npm t 
    "
    cd ..
    exit 0
  fi
  
  nix develop --command bash -c "
    set -e
    npm i
    npm t 
    "
  cd ..
}

if [[ -n "$SCOPE" ]]; then

  case "$SCOPE" in
  "hello_world")
    setup_and_build_hello_world
    ;;
  "holo_integration")
    rm -rf /tmp/holo-flake
    cd /tmp

    hc-scaffold web-app holo-flake --setup-nix true --template vue
    cd holo-flake

    nix develop --command bash -c "
      set -e
      # Check if holo-dev-server is in the path
      if command -v holo-dev-server >/dev/null 2>&1; then
        echo 'holo-dev-server is available in the PATH while it should not'
        exit 1
      else
        echo 'holo-dev-server is NOT available in the PATH'
      fi
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

    rm -rf /tmp/holo-flake
    cd /tmp
    ;;
  *)
    echo "Error: SCOPE must be one of 'hello_world', 'holo_integration', but got $SCOPE."
    exit 1
    ;;
  esac

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