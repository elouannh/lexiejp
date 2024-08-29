#!/bin/bash

ENV_FILE="envs/dev/.env"

cp ./envs/dev/.env ./.env

cargo run

if [ $? -ne 0 ]; then
    echo "Compilation failed."
    exit 1
fi

./target/debug/lexiejp
