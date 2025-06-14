#!/bin/bash

if [ -z "$1" ]; then
  echo "Usage: $0 <contest-name>"
  exit 1
fi

CONTEST="$1"

mkdir ${CONTEST}
cd ${CONTEST}
cargo generate --git https://github.com/zaki-yama-labs/atcoder-rust-base --name "${CONTEST}-a"
cargo generate --git https://github.com/zaki-yama-labs/atcoder-rust-base --name "${CONTEST}-b"
cargo generate --git https://github.com/zaki-yama-labs/atcoder-rust-base --name "${CONTEST}-c"
cargo generate --git https://github.com/zaki-yama-labs/atcoder-rust-base --name "${CONTEST}-d"
