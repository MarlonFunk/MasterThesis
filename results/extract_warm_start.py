## Script to filter results of concurrency test to include only warm starts

import json

# Read JSON data from a file
with open('/home/m/Documents/MasterTGN/MasterThesis/results/final/1_wasm_concurrency_mixed_75_25.log', 'r') as file:
    #data = json.load(file)
    data = file.read()

json_elements = data.split('\n')

for json_element in json_elements:
    if json_element.strip():  # Check if the line is not empty
        parsed_data = json.loads(json_element)
    # Filter the JSON data
        filtered_data = []
        for item in parsed_data:
            new_item = {
                "no_concurrent_requests": item["no_concurrent_requests"],
                "responses": []
            }
            for response in item["responses"]:
                annotations = response["annotations"]
                has_init_time = any(ann["key"] == "initTime" for ann in annotations)
                has_wait_time = any(ann["key"] == "waitTime" for ann in annotations)
                
                if has_wait_time and not has_init_time:
                    new_item["responses"].append(response)
                    
            if new_item["responses"]:
                filtered_data.append(new_item)

        # Write the filtered JSON data to a new file
        with open('3_wasm_concurrency_mixed_75_25.log', 'a') as file:
            json.dump(filtered_data, file, indent=2)