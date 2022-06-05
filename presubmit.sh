#!/bin/bash

set -e

# Run gometalinter in all go directories
for d in $(find . -name \*.go | sed -r 's|/[^/]+$||' |sort -u) ; do
    golint $d
    pushd $d
    golangci-lint run
    popd
done

pytype .

# Does not work very well:
# which docker && docker run --rm -e "WORKSPACE=${PWD}" -v "$PWD:/app" shiftleft/sast-scan scan
