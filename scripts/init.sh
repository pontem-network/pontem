#!/usr/bin/env bash

set -e

echo "*** Initializing WASM build environment"

REQUIRED_TOOLCHAIN=$(echo `rustup show active-toolchain` | awk '{print $1}')

if [ -z $CI_PROJECT_NAME ] ; then
   rustup update nightly
   rustup update $REQUIRED_TOOLCHAIN
   rustup update stable
fi

rustup target add wasm32-unknown-unknown --toolchain nightly
rustup target add wasm32-unknown-unknown --toolchain $REQUIRED_TOOLCHAIN
