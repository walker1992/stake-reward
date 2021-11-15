#!/usr/bin/env bash

# Script to setup a local solana-test-validator with the stake reward program

cd "$(dirname "$0")"

keys_dir=keys
mkdir -p $keys_dir
if test -f $validator_list
then
  rm $validator_list
fi

create_keypair () {
  if test ! -f $1
  then
    solana-keygen new --no-passphrase -s -o $1
  fi
}

build_program () {
  cargo build-bpf --manifest-path ../../program/Cargo.toml
}

setup_validator() {
  solana-test-validator --bpf-program 88gNHvxuPxaFTPELWBRYk59xCFqpjCt6MoBA1Lqk7qny ../../program/target/deploy/stake_reward.so --quiet --reset --slots-per-epoch 32 &
  pid=$!
  solana config set --url http://127.0.0.1:8899
  solana config set --commitment confirmed
  echo "waiting for solana-test-validator, pid: $pid"
  sleep 5
}

echo "Building on-chain program"
build_program

echo "Setting up local validator"
setup_validator

