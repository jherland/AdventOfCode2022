#!/bin/sh

set -e

day=$1
if test -n "$day"; then
  shift
else
  day=$(date +%d)
fi

cargo build --release --bin "day${day}"
time ./target/release/"day${day}" < "${day}.input"
