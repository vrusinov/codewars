#!/bin/bash

set -e

# Run gometalinter in all go directories
for d in $(find . -name \*.go | sed -r 's|/[^/]+$||' |sort -u) ; do
    golint $d
    gometalinter $d
done

pytype .

docker run --rm -e "WORKSPACE=${PWD}" -v "$PWD:/app" shiftleft/sast-scan scan --build