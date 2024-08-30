#!/bin/bash

ENV_FILE="envs/prod/.env"

cp ./envs/prod/.env ./.env

cargo build --release

if [ $? -ne 0 ]; then
    echo "Compilation failed."
    exit 1
fi

./target/release/lexiejp
