#!/bin/bash

if [[ $1 == 'cli' ]]; then
  cargo run --bin "cli" -- "$2" "$3"
elif [[ $1 == 'server' ]]; then
  cargo run --bin server
else
  echo "Oops!"
fi
