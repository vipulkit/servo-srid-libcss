#!/bin/sh

export CFG_DISABLE_MANAGE_SUBMODULES=1
mkdir -p build 
cd build
../configure
make clean
make
