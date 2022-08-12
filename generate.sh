#!/usr/bin/env sh

svd2rust --ignore_groups --target cortex-m --strict -i svd/LPC550x.xml && rm -rf src && form -i lib.rs -o src/ && rm lib.rs && sed -i '1i#![allow(clippy::all)]' src/lib.rs && cargo fmt
