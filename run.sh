#!/bin/sh

set -e

day=$1
if test -n "$day"; then
  shift
else
  day=$(date +%d)
fi

cat "${day}.input" | cargo run --bin "day${day}" $@
