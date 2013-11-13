#!/bin/sh

export CFG_DISABLE_MANAGE_SUBMODULES=1
export CFG_ENABLE_DEBUG=F
export VERBOSE=F
mkdir -p build 
cd build
../configure
make clean
make
