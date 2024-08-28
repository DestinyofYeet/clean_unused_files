#!/bin/bash

if [[ -z "$1" ]]; then
  printf "Invalid option, must be a valid platform: https://github.com/cross-rs/cross/tree/main/docker\n"
  exit
fi

cross build --target "$1" "$2"
