#!/bin/sh

cd src
find . -name "*.rs" -exec rustfmt {} \;
