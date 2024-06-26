#!/bin/bash

if [[ $1 == 'cli' ]]; then
  cargo run --bin "cli" -- -r
elif [[ $1 == 'server' ]]; then
  cargo run --bin server
else
  echo "Oops!"
fi
