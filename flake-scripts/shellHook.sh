#!/usr/bin/env bash

if command -v podman > /dev/null; then
    containerCommand=podman
elif command -v docker > /dev/null; then
    containerCommand=docker
else
    printf '%s\n' 'Install Podman or Docker to continue.'
    exit 1
fi

if ! printf '%s\n' 'exit' | \
    mysql --host=127.0.0.1 --user=mysql --password=mysql > /dev/null 2>&1; then

    $containerCommand load < "$1"

    if $containerCommand run --rm --detach -p 3306:3306 localhost/mysql:latest > /dev/null; then
        printf '%s\n' 'MySQL server started at 127.0.0.1:3306'
    else
        printf '%s\n' 'ERROR: Unable to start MySQL server! Exiting.'
        exit 1
    fi

    sleep 4

    mysql --host=127.0.0.1 --user=mysql --password=mysql < migrations/init.sql

else
    printf 'MySQL server found at 127.0.0.1:3306\n'
fi

export DATABASE_URL='mysql://mysql:mysql@127.0.0.1/mh_reg'

ln -sf "$(pwd)/hooks/pre-commit" "$(pwd)/.git/hooks/pre-commit"

export ROCKET_PORT="8001"
export ROCKET_ADDRESS="0.0.0.0"
export ROCKET_LIMITS='{ file = 101MiB }'
export ROCKET_DATABASES='{ mh_reg = { url = "mysql://mysql:mysql@127.0.0.1/mh_reg" } }'
export OPAQUE_DATA_DIR='storage'
