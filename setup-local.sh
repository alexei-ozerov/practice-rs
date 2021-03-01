#!/bin/bash

# Static Args
USER=$1
PASS=$2
ADDR=$3
PORT=$4
DB=$5

if [[ -z ${USER}  ]]; then
    echo "ERROR: USER is not provided."
    exit 1
fi

if [[ -z ${PASS}  ]]; then
    echo "ERROR: PASS is not provided."
    exit 1
fi

if [[ -z ${ADDR}  ]]; then
    echo "ERROR: ADDR is not provided."
    exit 1
fi

if [[ -z ${PORT}  ]]; then
    echo "ERROR: PORT is not provided."
    exit 1
fi

if [[ -z ${DB}  ]]; then
    echo "ERROR: DB is not provided."
    exit 1
fi

echo -e "You will need the following: USER, PASS, ADDRESS, PORT, DB NAME."
echo -e "An example: aozerov 0112 localhost 5433 echo_ms"
echo DATABASE_URL=postgres://${USER}:${PASS}}@${ADDR}}:${PORT}/${DB} >> .env
echo MS_ADDR=127.0.0.1:3000 >> .env
