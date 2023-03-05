#!/bin/sh

cd $1
cargo clippy --fix --allow-staged -- \
-W clippy::pedantic \
-W clippy::nursery \
-W clippy::cargo
