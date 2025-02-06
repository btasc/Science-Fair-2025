#!/bin/bash

cd ./Core-rs/wasm_export
wasm-pack build --target nodejs
cd ../../