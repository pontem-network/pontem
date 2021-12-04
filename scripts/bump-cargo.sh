#!/usr/bin/env bash

OLD=$1
NEW=$2
echo changing $1 to $2
sed -i "s/${1}/${2}/g" $(find -name Cargo.toml)

