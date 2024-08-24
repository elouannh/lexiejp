#!/bin/bash

ENV_FILE="envs/prod/.env"

if [ -f "$ENV_FILE" ]; then
	export $(grep -v '#^' $ENV_FILE | xargs)
else
	exit 1
fi

cargo build --release

if [ $? -ne 0 ]; then
    echo "Compilation failed."
    exit 1
fi

./target/release/lexiejp
