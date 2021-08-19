#!/bin/sh

# script to publish the crates 
# we added a 5s sleep in between publish to give time for dependency crate to propagate to crates.io
#

set -ev
cd crates/sauron-core && cargo publish && cd - && \
echo "sleeping for 5s" && sleep 5 &&\
cd crates/sauron-node-macro && cargo publish && cd - &&
echo "sleeping for 5s" && sleep 5 &&\
cargo publish
