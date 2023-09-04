## Script to run one specific test manually

#!/bin/bash

# First build
# cargo build --manifest-path ./ow-evaluation/Cargo.toml --release --bin cold-start-test 

#can be done together
# cargo build --manifest-path ./ow-evaluation/Cargo.toml --release --bin concurrency-test


if [ -z "$1" ]; then
    echo "test type required!"
    exit -1
else
    testtype=$1
    echo "Executing "$testtype" test!"
fi

if [ -z "$2" ]; then
    echo "platform required!"
    exit -1
else
    platform=$2
    echo "Executing test on "$platform"!"
fi

if [ -z "$3" ]; then
    echo "Without creating plots!"
else
    create_plots=$3
    
fi

if [ -z "$4" ]; then
    echo "Old data will not be deleted!"
else
    delete_data=$4
fi

if [ -z "$5" ]; then
    echo "Resources will not be measured!"
else
    if [ "$5" -eq 1 ]; then
      echo "Starting measurment of resources!"
      sudo /home/m/Documents/MasterTGN/MasterThesis/results/measure_resources.sh&
      measure_resources_pid=$(pgrep -f "measure_resources.sh")
    fi
fi

if [ "$6" == "hash" ]; then
    echo "Creating load using hash!"
    create_load=$6
elif [ "$6" == "mixed" ]; then
    echo "Creating mixed load!"
    create_load=$6
else
    echo "Without load!"
    create_load=0
fi


if [ $delete_data -eq 1 ]; then
echo "Deleting all logfiles!"
  rm 1_*
  rm 2_*
fi
export WHISK_AUTH=23bc46b1-71f6-4ed5-8c54-816aa4f8c502:123zO3xZCLrMN6v2BKK1dXYFpXlPkccOFqm12CdAsMgRU4VrNZ9lyGVCGuMDGIwP

if [ "$platform" == "wasm" ]; then
  action_source_folder="/home/m/Documents/MasterTGN/MasterThesis/OpenWhisk/openwhisk/wow/target/wasm32-wasi/release/examples/"
  action_create_parameter="--kind wasm:0.1"
  if [ "$testtype" == "cold-start" ]; then
    action_binary="hello-wasmtime.zip"
  elif [ "$testtype" == "concurrency" ];then
    if [ "$create_load" == "mixed" ]; then
      # For two actions define action_binary later
      echo ""
    elif [ "$create_load" == "hash" ]; then
      action_binary="hash-wasmtime.zip"
    else # no load
      action_binary="hello-wasmtime.zip"
    fi
  elif [ "$testtype" == "capacity" ] ;then
    action_binary="sleep-wasmtime.zip"
  fi
elif [ "$platform" == "docker" ]; then
  action_source_folder="/home/m/Documents/MasterTGN/MasterThesis/test/"
  action_create_parameter=""
  if [ "$testtype" == "cold-start" ]; then
    action_binary="hello.go"
  elif [ "$testtype" == "concurrency" ];then
    if [ "$create_load" == "mixed" ]; then
      # For two actions define action_binary later
      echo ""
    elif [ "$create_load" == "hash" ] ; then
      action_binary="hash.go"
    else # no load
      action_binary="hello.go"
    fi
  elif [ "$testtype" == "capacity" ];then
        action_binary="sleep.go"
  fi
fi

if [ "$testtype" == "cold-start" ]; then
    for (( i=0; i<=20; i++ )); do
        functionname="hello""$i"
        
        action_source_path="$action_source_folder""$action_binary"
        wsk action delete $functionname
        wsk action create $action_create_parameter $functionname $action_source_path  
        if [ $? -eq 0 ]; then
           echo "Action $functionname created from: $action_create_parameter $action_source_path";
        fi 
    done

    for (( i=1; i<=10; i++ )); do
    #TODO: In case of cold start additional sleep after one iteration
      /home/m/Documents/MasterTGN/MasterThesis/binaries/cold-start-test | tee -a /home/m/Documents/MasterTGN/MasterThesis/results/1_"$platform"_cold_start.log
      sleep 25
    done
    # Read JSON data from file and calculate the sum of initTime and waitTime for each activationId
    cat 1_"$platform"_cold_start.log | jq -r '.[] | .no_concurrent_requests as $numResponses | .responses[] | .activationId as $activationId | .annotations[] | select(.key == "initTime" or .key == "waitTime") | "\($numResponses) \($activationId) \(.value)"' |
    awk '
      {
        numResponses = $1;
        activationId = $2;
        value = $3;
        sum[activationId]["numResponses"] = numResponses;
        sum[activationId]["timeSum"] += value;
      }
      
      END {
        for (activationId in sum) {
          numResponses = sum[activationId]["numResponses"];
          timeSum = sum[activationId]["timeSum"];
          print activationId, numResponses, timeSum;
        }
      }
    ' >> /home/m/Documents/MasterTGN/MasterThesis/results/2_"$platform"_cold_start.log

    if [ $create_plots -eq 1 ]; then
        echo "Creating plot!"
        Rscript cold_start.R 2_"$platform"_cold_start.log > table_2_"$platform"_cold_start.log
    fi

elif [ "$testtype" == "concurrency" ]; then
    result_file_1=""
    result_file_2=""
    if [ "$create_load" == "mixed" ]; then
        functionnames=("net" "prime")
        # Print the elements of the array
        for functionname in "${functionnames[@]}"; do
            if [ "$platform" == "wasm" ]; then
               if [ "$functionname" == "net" ]; then
                  action_binary="net-wasmtime.zip"
               else
                  action_binary="prime-wasmtime.zip"
               fi
            else
                if [ "$functionname" == "net" ]; then
                  action_binary="net.go"
               else
                  action_binary="prime.go"
               fi
            fi
            action_source_path="$action_source_folder""$action_binary"
            wsk action delete $functionname
            wsk action create $action_create_parameter $functionname $action_source_path  
            if [ $? -eq 0 ]; then
                echo "Action $functionname created from: $action_create_parameter $action_source_path";
            fi 
            wsk action invoke --blocking $functionname
        done
        result_file_1="/home/m/Documents/MasterTGN/MasterThesis/results/1_"$platform"_concurrency_mixed.log"
        result_file_2="/home/m/Documents/MasterTGN/MasterThesis/results/2_"$platform"_concurrency_mixed.log"
        # /home/m/Documents/MasterTGN/MasterThesis/binaries/concurrency-test-mixed-"$platform" | tee -a "$result_file_1" 
        
        for (( i=1; i<=10; i++ )); do
          /home/m/Documents/MasterTGN/MasterThesis/binaries/concurrency-test-mixed_1_9 | tee -a "$result_file_1" 
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
          result_file_1="/home/m/Documents/MasterTGN/MasterThesis/results/1_"$platform"_concurrency_"$functionname".log"
          result_file_2="/home/m/Documents/MasterTGN/MasterThesis/results/2_"$platform"_concurrency_"$functionname".log"
          action_source_path="$action_source_folder""$action_binary"
          wsk action delete $functionname
          wsk action create $action_create_parameter $functionname $action_source_path  
          if [ $? -eq 0 ]; then
              echo "Action $functionname created from: $action_create_parameter $action_source_path";
          fi 
          wsk action invoke --blocking $functionname $action_invoke_parameter

          for (( i=1; i<=10; i++ )); do
            # /home/m/Documents/MasterTGN/MasterThesis/binaries/concurrency-test-"$functionname"-"$platform" | tee -a "$result_file_1"
              /home/m/Documents/MasterTGN/MasterThesis/binaries/concurrency-test-"$functionname" | tee -a "$result_file_1"
          done
    fi
      

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
    ' >> "$result_file_2"


  ###########################################

    activation_id_count=$(jq '.[].responses[].activationId' "$result_file_1"| sort -u | wc -l)
    success_count=$(jq '.[].responses[] | select(.response.status == "success" and .response.success == true) | .response.success' "$result_file_1" | grep -c "true")
    echo "activations: "$activation_id_count""
    echo "sucessfull: "$success_count""
  ########################################### 

    if [ $create_plots -eq 1 ]; then
        echo "Creating plot!"
        Rscript concurrency.R "$result_file_2" > table_2_"$platform"_concurrency_mixed.log
    fi

elif [ "$testtype" == "capacity" ]; then
    # action creating same as for concurrency. Create some kind of function?
    functionname="sleep"
    action_source_path="$action_source_folder""$action_binary"
    wsk action delete $functionname
    wsk action create $action_create_parameter $functionname $action_source_path  
    if [ $? -eq 0 ]; then
        echo "Action $functionname created from: $action_create_parameter $action_source_path";
    fi 
    # for (( i=1; i<=10; i++ )); do
      /home/m/Documents/MasterTGN/MasterThesis/binaries/capacity-test-"$platform" |  tee -a /home/m/Documents/MasterTGN/MasterThesis/results/1_"$platform"_capacity.log
    # done
  cat 1_"$platform"_capacity.log | jq -r '.[] | .no_concurrent_requests as $numRequests | .responses[].annotations[] | select(.key == "waitTime") | [$numRequests, .value] | @tsv' | awk '
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
' >> 2_"$platform"_capacity.log

highest_concurrency=$(cat 1_"$platform"_capacity.log | jq '.[].no_concurrent_requests' | sort -rn | head -n1)
echo "$highest_concurrency"
longest_duration=$(cat 1_"$platform"_capacity.log | jq '[.[] | .responses[] | .duration] | max')
echo "$longest_duration"

  # Count occurrences of the specific response fields
  success_count=0
  activation_id_count=0

  if [ "$platform" == "docker" ]; then
    success_count=$(jq '.[].responses[] | select(.response.status == "success" and .response.success == true) | .response.success' "$result_file_1" | grep -c "true")
    activation_id_count=$(jq '.[].responses[].activationId' 1_"$platform"_capacity.log| sort -u | wc -l)
  else
    success_count=$(jq '.[].responses[] | select(.response.result.status == "success" and .response.result.status_code == 0 and .response.result.success == true) | .response.success' 1_"$platform"_capacity.log | grep -c "true")
    activation_id_count=$(jq '.[].responses[].activationId' 1_"$platform"_capacity.log| sort -u | wc -l)
  fi
  # echo "Occurrences of success response: $success_count"
  # echo "Number of unique activationId: $activation_id_count"
  percentage=$((( $success_count / $activation_id_count ) *100))
  echo -e "Sucess percentage: \t\t\t $percentage "
  echo -e "Amount of activations: \t\t\t $activation_id_count"
  echo -e "Amount of successfull activations: \t $success_count"

else
  echo "Wrong testtype!"
fi

if [ "$5" -eq 1 ]; then
  kill "$measure_resources_pid"
fi

# ## Docker - cold start
# # docker - test/hello.go
# for (( i=1; i<=10; i++ )); do     
#     functionname="hello""$i";     
#     wsk action create $functionname hello.go ;     
#     echo "Action $functionname created"; 
# done

# # In ~/Documents/MasterTGN/MasterThesis/OpenWhisk/openwhisk/wow/target/release execute 
# ./cold-start-test | tee -a ~/Documents/MasterTGN/MasterThesis/results/docker_cold_start_tests.json | jq -r '.[].responses | length as $responses | .[].annotations[] | select(.key == "initTime" or .key == "waitTime") | "\($responses) responses - \(.key): \(.value)"' >> docker_data.log
# #cat docker_cold_start_tests.json | jq -r '.[].responses | length as $responses | .[].annotations[] | select(.key == "initTime" or .key == "waitTime") | "\($responses) responses - \(.key): \(.value)"' >> docker_data.log



# ## Docker - concurrency

# # docker - test/hello.go
# functionname="hello" 
# wsk action create $functionname ~/Documents/MasterTGN/MasterThesis/test/hello.go

# ~/Documents/MasterTGN/MasterThesis/OpenWhisk/openwhisk/wow/target/release/concurrency-test | tee -a ~/Documents/MasterTGN/MasterThesis/results/docker_concurrency_tests.json 

# ./cold-start-test  | jq -r '.[].responses | length as $responses | .[].annotations[] | select(.key == "initTime" or .key == "waitTime") | "\($responses) responses - \(.key): \(.value)"' >> 2docker_data.log


# # echo "initTimes:" >> docker_data.log
# cat docker_cold_start_tests.json | jq -r '.[].responses | length as $responses | .[].annotations[] | select(.key == "initTime") | "\($responses) responses - \(.key): \(.value)"' >> docker_data.log

# # echo "waitTimes:" >> docker_data.log
# cat docker_cold_start_tests.json | jq -r '.[].responses | length as $responses | .[].annotations[] | select(.key == "waitTime") | "\($responses) responses - \(.key): \(.value)"' >> docker_data.log




# #!/bin/bash

# # Read JSON data from file and calculate the sum of initTime and waitTime for each activationId
# #!/bin/bash
