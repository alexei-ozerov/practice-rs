#!/bin/bash

DOCKER_BUILDKIT=1 docker build  ms/. -t practice-rs:ms
docker-compose up -d
