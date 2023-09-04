## Script to call R script and save results in table format for report

#!/bin/bash


list_docker="2_docker_concurrency_hash.log 2_docker_concurrency_hello.log 2_docker_concurrency_mixed.log" #2_docker_cold_start.log
list_wasm="2_wasm_concurrency_hash.log 2_wasm_concurrency_hello.log 2_wasm_concurrency_mixed.log" #2_wasm_cold_start.log


for i in $list_wasm; do 
    Rscript concurrency.R final/"$i">table_"$i"
    # echo "Rscript concurrency.R 'final/"$i"' > table_"$i".log"

done