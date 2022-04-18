#!/bin/bash

set -e

# Run gometalinter in all go directories
for d in $(find . -name \*.go | sed -r 's|/[^/]+$||' |sort -u) ; do
    gometalinter $d
done

pytype .