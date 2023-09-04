## Script to create plots

#!/bin/bash

folder_path="/home/m/MasterThesis/results/final"
cd "$folder_path"

file_regex="4_*"

for file_path in "$folder_path"/*; do
    if [[ -f "$file_path" && "$file_path" =~ $file_regex ]]; then
        base_name=$(basename "$file_path")
        Rscript ../concurrency.R $file_path #> table_$base_name
    fi
done

#