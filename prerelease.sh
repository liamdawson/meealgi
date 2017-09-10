#!/bin/bash

# abort on unset variable, or error output
set -e
set -u

COMMAND="$0"
USAGE="$(cat <<EOF
usage: $COMMAND [VERSION]

  VERSION  new version number
EOF
)"

if [[ $# -ne 1 ]]
then
  echo "$USAGE"
  exit 1
fi

if ! which changelog-rs >/dev/null 2>&1
then
  echo "changelog-rs must be installed."
  exit 3
fi

NEW_VERSION="${1//v/}"
LAST_VERSION="$(git tag -l 'v*' | tail -n1)"

if [[ ! "$NEW_VERSION" =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]
then
  echo "invalid version: expected in format [MAJOR].[MINOR].[PATCH]"
  echo "  e.g. 1.0.5"

  exit 2
fi

# first, ensure tests pass locally
cargo test

# build the changelog
# (relies on https://github.com/perlun/changelog-rs/pull/15)
changelog-rs --to-alias "$NEW_VERSION" . "$LAST_VERSION" HEAD >> CHANGELOG.md

# allow manual changelog massaging
"$EDITOR" CHANGELOG.md

# set new package version
sed -i -e 's/^version = ".*"$/version = "'"$NEW_VERSION"'"/' Cargo.toml

git add CHANGELOG.md Cargo.toml

echo "version $NEW_VERSION should be ready for release:"
echo
echo "  git commit -m 'release version v$NEW_VERSION'"
echo "  git tag -a v$NEW_VERSION"
