#!/bin/bash

target=$1

folder=$2

if [[ -z "$target" ]]; then
  printf "You need to provide a target!\n"
  exit
fi

if [[ -z "$folder" ]]; then
  printf "You need to provide a folder to upload to!\n"
  exit
fi

scp "/drives/SPEED/ProgrammingStuff/rust/clean_unused_files/target/aarch64-unknown-linux-gnu/debug/clean_unused_files" "$target:$folder/clean_unused_files"

# shellcheck disable=SC2029
ssh "$target" "chmod +x $folder/clean_unused_files && cd $folder && RUST_BACKTRACE=1 RUST_BACKTRACE=full $folder/clean_unused_files"