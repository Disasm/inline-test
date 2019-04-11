#!/bin/bash

set -euxo pipefail

crate=test

# remove existing blobs because otherwise this will append object files to the old blobs
rm -f bin/*.a

clang asm.c -flto=thin -c -o bin/$crate.o -O2 --target=thumbv7m-none-eabi
ar crs bin/thumbv7m-none-eabi.a bin/$crate.o

rm bin/$crate.o
