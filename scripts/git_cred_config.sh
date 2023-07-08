#!/usr/bin/env bash
# Configures git credentials for the container from a env var
# -- ENV -- #
# GIT_TOKEN - the token to use for git
# -- ENV -- #

check_for_deps() {
  deps=(
    git
  )

 for dep in "${deps[@]}"; do
   if [ ! "$(command -v $dep)" ]
   then
    echo "dependency [$dep] not found. exiting"
    exit 1
   fi
 done
}
check_for_deps

# get the token from the env var
GIT_TOKEN=${GIT_TOKEN:-}

git config --global credential.helper '!f() { sleep 1; echo "username=shoppa"; echo "password=${GIT_TOKEN}"; }; f'

echo "git credentials configured"
