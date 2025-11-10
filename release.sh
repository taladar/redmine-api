#!/bin/bash

set -e -u

if [[ $# -lt 1 ]]; then
  echo "Usage: $0 <major|minor|patch>" >&2
  exit 1
fi

level="$1"

cargo set-version --bump "${level}"

version="$(cargo get package.version)"

git cliff --prepend CHANGELOG.md -u -t "redmine_api_${version}"

rumdl fmt --fix CHANGELOG.md

cargo build

git add CHANGELOG.md Cargo.toml

git commit -m "chore(release): Release version ${version}"

git tag "redmine_api_${version}"

for remote in origin github taladar; do
  git push "${remote}"
  git push "${remote}" "redmine_api_${version}"
done

cargo publish --dry-run
