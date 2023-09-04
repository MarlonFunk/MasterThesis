#!/bin/bash

## Script to run all tests automatically, including the creation of actions, switch to log resources and switch to create plots

export WHISK_AUTH=23bc46b1-71f6-4ed5-8c54-816aa4f8c502:123zO3xZCLrMN6v2BKK1dXYFpXlPkccOFqm12CdAsMgRU4VrNZ9lyGVCGuMDGIwP



if [ -z "$1" ]; then
    echo "Old data will not be deleted!"
else
    delete_data=$1
fi

if [ -z "$2" ]; then
    echo "Without creating plots!"
else
    create_plots=$2
    
fi

if [ $delete_data -eq 1 ]; then
echo "Deleting all logfiles!"
  rm 1_*
  rm 2_*
fi



# ## Wasm
# #cold start
# action_source_folder="/home/m/Documents/MasterTGN/MasterThesis/OpenWhisk/openwhisk/wow/target/wasm32-wasi/release/examples/"
# action_create_parameter="--kind wasm:0.1"
# action_binary="hello-wasmtime.zip"
# for (( i=0; i<=16; i++ )); do
#     functionname="hello""$i"
    
#     action_source_path="$action_source_folder""$action_binary"
#     wsk action delete $functionname
#     wsk action create $action_create_parameter $functionname $action_source_path  
#     if [ $? -eq 0 ]; then
#         echo "Action $functionname created from: $action_create_parameter $action_source_path";
#     fi 
# done
# for (( i=1; i<=10; i++ )); do
#     /home/m/Documents/MasterTGN/MasterThesis/binaries/cold-start-test | tee -a /home/m/Documents/MasterTGN/MasterThesis/results/1_wasm_cold_start.log
#     sleep 25
# done
# # Read JSON data from file and calculate the sum of initTime and waitTime for each activationId
# cat 1_wasm_cold_start.log | jq -r '.[] | .no_concurrent_requests as $numResponses | .responses[] | .activationId as $activationId | .annotations[] | select(.key == "initTime" or .key == "waitTime") | "\($numResponses) \($activationId) \(.value)"' |
# awk '
#     {
#         numResponses = $1;
#         activationId = $2;
#         value = $3;
#         sum[activationId]["numResponses"] = numResponses;
#         sum[activationId]["timeSum"] += value;
#     }
    
#     END {
#         for (activationId in sum) {
#             numResponses = sum[activationId]["numResponses"];
#             timeSum = sum[activationId]["timeSum"];
#             print activationId, numResponses, timeSum;
#         }
#     }
# ' >> /home/m/Documents/MasterTGN/MasterThesis/results/2_wasm_cold_start.log

# if [ $create_plots -eq 1 ]; then
#     echo "Creating plot!"
#     Rscript cold_start.R 2_wasm_cold_start.log > table_2_wasm_cold_start.log
# fi

# #concurrency
# #no-load
# for create_load in "hash" "mixed" "hello"; do
#     echo "-------------------__"
#     if [ "$create_load" == "mixed" ]; then
#         # For two actions define action_binary later
#         echo ""
#     elif [ "$create_load" == "hash" ]; then
#         action_binary="hash-wasmtime.zip"
#     else # no load
#         action_binary="hello-wasmtime.zip"
#     fi
#     result_file_1=""
#     result_file_2=""
#     if [ "$create_load" == "mixed" ]; then
#         functionnames=("net" "prime")
#         # Print the elements of the array
#         for functionname in "${functionnames[@]}"; do
#             if [ "$functionname" == "net" ]; then
#                 action_binary="net-wasmtime.zip"
#             else
#                 action_binary="prime-wasmtime.zip"
#             fi
  
#             action_source_path="$action_source_folder""$action_binary"
#             wsk action delete $functionname
#             wsk action create $action_create_parameter $functionname $action_source_path  
#             if [ $? -eq 0 ]; then
#                 echo "Action $functionname created from: $action_create_parameter $action_source_path";
#             fi 
#             wsk action invoke --blocking $functionname
#         done
    
#         for type in "_1_9" "_25_75" "_5_5" "_75_25" "_9_1"; do
#             result_file_1="/home/m/Documents/MasterTGN/MasterThesis/results/1_wasm_concurrency_mixed""$type"".log"
#             result_file_2="/home/m/Documents/MasterTGN/MasterThesis/results/2_wasm_concurrency_mixed""$type"".log"
#             for (( i=1; i<=10; i++ )); do
#                 /home/m/Documents/MasterTGN/MasterThesis/binaries/concurrency-test-mixed"$type" | tee -a "$result_file_1"
#             done

#             cat "$result_file_1" | jq -r '.[] | .no_concurrent_requests as $numRequests | .responses[].annotations[] | select(.key == "waitTime") | [$numRequests, .value] | @tsv' | awk '
#             {
#             numRequests = $1;
#             waitTime = $2;
#             waitTimes[numRequests] = waitTimes[numRequests] ", " waitTime;
#             }

#             END {
#             for (numRequests in waitTimes) {
#                 printf("%d: %s\n", numRequests, waitTimes[numRequests]);
#             }
#             }
#             ' > "$result_file_2"

#             Rscript concurrency.R "$result_file_2" > table_2_wasm_concurrency_mixed.log

#         done
#     else #Not with mixed load
#         if [ "$create_load" == "hash" ]; then
#             functionname="hash"
#             action_invoke_parameter="--param iterations 100000 --param input 124k2l35j" # random input, doesnt matter
#         else
#             # No load
#             functionname="hello"
#             action_invoke_parameter="--param input there"
#         fi
#         result_file_1="/home/m/Documents/MasterTGN/MasterThesis/results/1_wasm_concurrency_"$functionname".log"
#         result_file_2="/home/m/Documents/MasterTGN/MasterThesis/results/2_wasm_concurrency_"$functionname".log"
#         action_source_path="$action_source_folder""$action_binary"
#         wsk action delete $functionname
#         wsk action create $action_create_parameter $functionname $action_source_path  
#         if [ $? -eq 0 ]; then
#             echo "Action $functionname created from: $action_create_parameter $action_source_path";
#         fi 
#         wsk action invoke --blocking $functionname $action_invoke_parameter

#         for (( i=1; i<=10; i++ )); do
#         /home/m/Documents/MasterTGN/MasterThesis/binaries/concurrency-test-"$functionname" | tee -a "$result_file_1"
#         done
    
    

    #     cat "$result_file_1" | jq -r '.[] | .no_concurrent_requests as $numRequests | .responses[].annotations[] | select(.key == "waitTime") | [$numRequests, .value] | @tsv' | awk '
    #     {
    #     numRequests = $1;
    #     waitTime = $2;
    #     waitTimes[numRequests] = waitTimes[numRequests] ", " waitTime;
    #     }

    #     END {
    #     for (numRequests in waitTimes) {
    #         printf("%d: %s\n", numRequests, waitTimes[numRequests]);
    #     }
    #     }
    #     ' > "$result_file_2"

    #     Rscript concurrency.R "$result_file_2" > table_2_wasm_concurrency_mixed.log

#     fi
# done

## docker
#cold start
action_source_folder="/home/m/Documents/MasterTGN/MasterThesis/test/"

action_create_parameter=""


#concurrency
#no-load
for create_load in "hash" "mixed" "hello"; do
    echo "-------------------__"
    if [ "$create_load" == "mixed" ]; then
        # For two actions define action_binary later
        echo ""
    elif [ "$create_load" == "hash" ]; then
        action_binary="hash.go"
    else # no load
        action_binary="hello.go"
    fi
    result_file_1=""
    result_file_2=""
    functionname=""
    if [ "$create_load" == "mixed" ]; then
        functionnames=("net" "prime")
        # Print the elements of the array
        for functionname in "${functionnames[@]}"; do
            if [ "$functionname" == "net" ]; then
                action_binary="net.go"
            else
                action_binary="prime.go"
            fi
  
            action_source_path="$action_source_folder""$action_binary"
            wsk action delete $functionname
            wsk action create $action_create_parameter $functionname $action_source_path  
            if [ $? -eq 0 ]; then
                echo "Action $functionname created from: $action_create_parameter $action_source_path";
            fi 
            wsk action invoke --blocking $functionname
        done
    
        for type in "_1_9" "_25_75" "_5_5" "_75_25" "_9_1"; do
            result_file_1="/home/m/Documents/MasterTGN/MasterThesis/results/1_docker_concurrency_mixed""$type"".log"
            result_file_2="/home/m/Documents/MasterTGN/MasterThesis/results/2_docker_concurrency_mixed""$type"".log"
            for (( i=1; i<=10; i++ )); do
                /home/m/Documents/MasterTGN/MasterThesis/binaries/concurrency-test-mixed"$type" | tee -a "$result_file_1"
            done

            cat "$result_file_1" | jq -r '.[] | .no_concurrent_requests as $numRequests | .responses[].annotations[] | select(.key == "waitTime") | [$numRequests, .value] | @tsv' | awk '
            {
            numRequests = $1;
            waitTime = $2;
            waitTimes[numRequests] = waitTimes[numRequests] ", " waitTime;
            }

            END {
            for (numRequests in waitTimes) {
                printf("%d: %s\n", numRequests, waitTimes[numRequests]);
            }
            }
            ' > "$result_file_2"

            if [ $create_plots -eq 1 ]; then
                Rscript concurrency.R "$result_file_2" > table_2_docker_concurrency_mixed"$type".log
            fi
        done
    else #Not with mixed load
        if [ "$create_load" == "hash" ]; then
            functionname="hash"
            action_invoke_parameter="--param iterations 100000 --param input 124k2l35j" # random input, doesnt matter
        else
            # No load
            functionname="hello"
            action_invoke_parameter="--param input there"
        fi
        result_file_1="/home/m/Documents/MasterTGN/MasterThesis/results/1_docker_concurrency_"$functionname".log"
        result_file_2="/home/m/Documents/MasterTGN/MasterThesis/results/2_docker_concurrency_"$functionname".log"
        action_source_path="$action_source_folder""$action_binary"
        wsk action delete $functionname
        wsk action create $action_create_parameter $functionname $action_source_path  
        if [ $? -eq 0 ]; then
            echo "Action $functionname created from: $action_create_parameter $action_source_path";
        fi 
        wsk action invoke --blocking $functionname $action_invoke_parameter

        for (( i=1; i<=10; i++ )); do
            /home/m/Documents/MasterTGN/MasterThesis/binaries/concurrency-test-"$functionname" | tee -a "$result_file_1"
        done
    
    

        cat "$result_file_1" | jq -r '.[] | .no_concurrent_requests as $numRequests | .responses[].annotations[] | select(.key == "waitTime") | [$numRequests, .value] | @tsv' | awk '
        {
        numRequests = $1;
        waitTime = $2;
        waitTimes[numRequests] = waitTimes[numRequests] ", " waitTime;
        }

        END {
        for (numRequests in waitTimes) {
            printf("%d: %s\n", numRequests, waitTimes[numRequests]);
        }
        }
        ' > "$result_file_2"
        if [ $create_plots -eq 1 ]; then
            Rscript concurrency.R "$result_file_2" > table_2_docker_concurrency_"$functionname".log
        fi
    fi
done

# action_binary="hello.go"

# for (( i=0; i<=16; i++ )); do
#     functionname="hello""$i"
    
#     action_source_path="$action_source_folder""$action_binary"
#     wsk action delete $functionname
#     wsk action create $action_create_parameter $functionname $action_source_path  
#     if [ $? -eq 0 ]; then
#         echo "Action $functionname created from: $action_create_parameter $action_source_path";
#     fi 
# done
# for (( i=1; i<=10; i++ )); do
#     /home/m/Documents/MasterTGN/MasterThesis/binaries/cold-start-test | tee -a /home/m/Documents/MasterTGN/MasterThesis/results/1_docker_cold_start.log
#     sleep 25
# done
# # Read JSON data from file and calculate the sum of initTime and waitTime for each activationId
# cat 1_docker_cold_start.log | jq -r '.[] | .no_concurrent_requests as $numResponses | .responses[] | .activationId as $activationId | .annotations[] | select(.key == "initTime" or .key == "waitTime") | "\($numResponses) \($activationId) \(.value)"' |
# awk '
#     {
#         numResponses = $1;
#         activationId = $2;
#         value = $3;
#         sum[activationId]["numResponses"] = numResponses;
#         sum[activationId]["timeSum"] += value;
#     }
    
#     END {
#         for (activationId in sum) {
#             numResponses = sum[activationId]["numResponses"];
#             timeSum = sum[activationId]["timeSum"];
#             print activationId, numResponses, timeSum;
#         }
#     }
# ' >> /home/m/Documents/MasterTGN/MasterThesis/results/2_docker_cold_start.log

# if [ $create_plots -eq 1 ]; then
#     echo "Creating plot!"
#     Rscript cold_start.R 2_docker_cold_start.log > table_2_docker_cold_start.log
# fi