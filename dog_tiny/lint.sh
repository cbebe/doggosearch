#!/bin/sh
#
cargo clippy -- \
-W clippy::pedantic \
-W clippy::nursery \
-W clippy::cargo
