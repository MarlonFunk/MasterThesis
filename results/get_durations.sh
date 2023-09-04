## Script to create all actions

#!/bin/bash

actions=("hash" "hello" "net" "prime" "sleep")
for functionname in "${actions[@]}"; do 
    wsk action create --kind wasm:0.1 $functionname /home/m/Documents/MasterTGN/MasterThesis/OpenWhisk/openwhisk/wow/target/wasm32-wasi/release/examples/"$functionname"-wasmtime.zip
     
done