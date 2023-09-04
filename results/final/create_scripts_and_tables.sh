#!/bin/bash

replacement_letter="2"

regex_pattern="2_docker_conc*.log"

# Loop over files matching the regex pattern
for result_file_1 in $(find  -type f -name "*$regex_pattern*"); do
    echo "$result_file_1"
    result_file_2="${replacement_letter}${result_file_1:3}"
    echo "$result_file_2"
    # cat "$result_file_1" | jq -r '.[] | .no_concurrent_requests as $numRequests | .responses[].annotations[] | select(.key == "waitTime") | [$numRequests, .value] | @tsv' | awk '
    # {
    # numRequests = $1;
    # waitTime = $2;
    # waitTimes[numRequests] = waitTimes[numRequests] ", " waitTime;
    # }

    # END {
    # for (numRequests in waitTimes) {
    #     printf("%d: %s\n", numRequests, waitTimes[numRequests]);
    # }
    # }
    # ' >> "$result_file_2"

    #> table_"$result_file_2"
    # Rscript ../concurrency.R "$result_file_2" 
    Rscript ../cold_start.R "$result_file_2">table_"$result_file_2"

done