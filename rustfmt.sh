#!/bin/sh

cd src
#find . -name "*.rs" -exec rustfmt {} \;
rustfmt db/queries.rs
