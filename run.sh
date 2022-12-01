#!/bin/sh

set -e

day=$1
shift

cat "${day}.input" | cargo run --bin "day${day}" $@
