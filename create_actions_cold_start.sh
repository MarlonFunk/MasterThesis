#!/bin/bash

## Script to create actions for manual cold-start test

desired_directory="/home/m/Documents/MasterTGN/MasterThesis/OpenWhisk/openwhisk/wow"

current_directory=$(pwd)

if [ "$current_directory" != "$desired_directory" ]; then
    echo "Current directory is not the desired directory. Changing directory..."
    cd "$desired_directory"
    echo "Changed directory to: $(pwd)"
else
    echo "Current directory is already the desired directory: $desired_directory"
fi

if [ -z "$1" ]; then
    echo "action name and docker or wasm required!"
fi

if [ -z "$2" ]; then
    echo "action name and docker or wasm required!"
fi

if [ "$2" == "wasm" ]; then
    echo "$2"
    echo "Creating for wasm"
    for (( i=1; i<=41; i++ )); do
        functionname="$1""$i"
        wsk action delete $functionname
        wsk action create --kind wasm:0.1 $functionname ./target/wasm32-wasi/release/examples/"$1"-wasmtime.zip  
        if [ $? -eq 0 ]; then
            echo "Action $functionname created";
        fi 
    done
elif [ "$2" == "docker" ]; then
    # Docker
    for (( i=1; i<=10; i++ )); do     
        functionname="hello""$i";  
        wsk action delete $functionname   
        wsk action create $functionname /home/m/Documents/MasterTGN/MasterThesis/test/hello.go ;     
        if [ $? -eq 0 ]; then
            echo "Action $functionname created";
        fi         
    done
else
    echo "Wrong input for second parameter!"
fi