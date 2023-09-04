#!/bin/bash

## Script to build Rust actions automatically

desired_directory="/home/m/Documents/MasterTGN/MasterThesis/OpenWhisk/openwhisk/wow"

current_directory=$(pwd)

if [ "$current_directory" != "$desired_directory" ]; then
    echo "Current directory is not the desired directory. Changing directory..."
    cd "$desired_directory"
    echo "Changed directory to: $(pwd)"
else
    echo "Current directory is already the desired directory: $desired_directory"
fi

functionname=$1

if [ -z "$2" ]; then
    create_function=0
else
    create_function=$2
fi


# Not neccessary, use wsk action update
if [ -z "$3" ]; then
    delete_function=0
else
    delete_function=$2
fi

if [ $delete_function -eq 1 ]; then
    echo "Deleting function $1"
    wsk action delete $1
fi

cargo build --manifest-path ./ow-wasm-action/Cargo.toml --release --example $functionname --target wasm32-wasi --no-default-features --features wasm

 

wasm-opt -O4 -o ./target/wasm32-wasi/release/examples/"$functionname".wasm ./target/wasm32-wasi/release/examples/"$functionname".wasm

 
cargo run --manifest-path ./ow-wasmtime-precompiler/Cargo.toml --release --bin wasmtime ./target/wasm32-wasi/release/examples/"$functionname".wasm 

 
echo "To create function without additional parameters add 1 as 2nd argument"

if [ $create_function -eq 1 ]; then
    wsk action create --kind wasm:0.1 $functionname ./target/wasm32-wasi/release/examples/"$functionname"-wasmtime.zip  
    echo "Action $functionname created"
fi

