#!/bin/bash
#
cargo new --vcs=none rust/$1
mkdir -p rust/$1/src/bin perl/$1 inputs/$1
cp part.rs rust/$1/src/bin/part1.rs
rm rust/$1/src/main.rs
cp part.pl perl/$1/part1.pl
touch inputs/$1/inputs1.txt
