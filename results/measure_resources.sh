#!/bin/bash

#echo "-------------------" >> log_res.log # Mark beginning of new execution without loosing old data
# if [ "$1" == "wasm" ]; then
#     outfile="wasm_res.log"
# else
#     outfile="docker_res.log"
# fi
    
    outfile="wasm_res.log"
    
    # echo "SPID STAT %CPU RSS VSZ" >> "$outfile" #TODO: include tabs
    # while [ true ]; do # Keep logging until cancelled
    #     stastats_executorts=$(sudo ps -o spid,stat,%cpu,rss,vsz= -C "executor" ) # Gives a list of the states of all threads and information about resources
    #     running=$(echo "$stats_executor" | awk '$2 ~ /R/ {print}') # Threads whose stat contains R
    #     # Only log if a thread is running
    #     if [[ ! -z "$running" ]] ; then
    #         # echo $running >> log_res.log

    #         while IFS= read -r thread_info; do
    #             echo "$thread_info" >> "$outfile"
    #         done <<< "$running"

    #     fi
    # done  

    outfile="docker_res.log"
    while [ true ]; do # Keep logging until cancelled
        docker_stats_output=$(docker stats --no-stream --format "table {{.Name}}\t{{.CPUPerc}}\t{{.MemUsage}}")

        cpu_sum=$(echo "$docker_stats_output" | awk 'NR > 1 { total += $2 } END { print total }')
        mem_sum=$(echo "$docker_stats_output" | awk 'NR > 1 { sub(/[A-Za-z]+/, "", $3); total += $3 } END { print total "MiB" }')

        echo "CPU%: $cpu_sum"" Memory: $mem_sum" >> "$outfile"
    done



 # thread_list=$( ps -T -o pid,stat,cpu,cp,rss,vsz= -C "executor" ) # Gives a list of the states of all threads and informations about resources
# running_thread_pid=$(echo "$thread_list" | awk '$2 ~ /R/ {print}')
# echo $running_thread_pid


    # while [[ "$process_state" == *"R"* ]]; do # At least one of the threads is running
    #     # All threads share the memory
    #     echo $(ps u | grep executo[r] | awk '{print $3 "\t" $5 "\t" $6}') >> log_res.log
    #     # sleep 0.1
    #     process_state=$( ps -T -o stat= -C "executor" )
    # done


