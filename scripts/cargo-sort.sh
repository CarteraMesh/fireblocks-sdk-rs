#!/bin/bash

set -e
set -o pipefail

PACKAGES=$(cargo metadata --format-version 1 --no-deps | jq -r '.packages[] | select(.targets[] | .kind[] == "lib" or .kind[] == "bin") | .manifest_path')

for package in $PACKAGES; do
    if ! cargo sort -c -g "$(dirname "$package")"
    then
        if [ "$1" == "--fix" ]
        then
            cargo sort -p -g "$(dirname "$package")"  | grep -E  -v Checking > /tmp/t && \
            mv -f -v  /tmp/t $package
        else
            exit 1
        fi
    fi

done
