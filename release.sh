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

# break list entries (one is generated per commit) with long lines
# over 80 characters at the last space before character 80 and indent
# them so they are still part of the list entry
list_item_re='^- '
while read -r line; do
  if [[ "${line}" =~ ${list_item_re} ]]; then
    if [[ ${#line} -gt 80 ]]; then
      next="${line:0:80}"
      next_without_suffix="${next% *}"
      if [[ "${next}" == "${next_without_suffix}" ]]; then
        # can't break this line, no spaces
        echo "${line}"
        continue
      fi
      echo "${next_without_suffix}"
      next_len=${#next_without_suffix}
      next_len=$((next_len + 1))
      rest="${line:${next_len}}"
      while [[ -n "${rest}" ]]; do
        next="${rest:0:78}"
        next_without_suffix="${next% *}"
        if [[ "${next}" == "${next_without_suffix}" ]]; then
          # can't break the rest, no spaces
          echo "  ${rest}"
          continue 2
        fi
        echo "  ${next_without_suffix}"
        next_len=${#next_without_suffix}
        next_len=$((next_len + 1))
        rest="${rest:${next_len}}"
      done
    else
      echo "${line}"
    fi
  else
    echo "${line}"
  fi
done <CHANGELOG.md | sponge CHANGELOG.md

cargo build

git add CHANGELOG.md Cargo.toml

git commit -m "chore(release): Release version ${version}"

git tag "redmine_api_${version}"

for remote in origin github taladar; do
  echo git push "${remote}"
  echo git push "${remote}" "redmine_api_${version}"
done

cargo publish --dry-run
