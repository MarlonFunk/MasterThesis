## Script to calculate mean of timestamps

import numpy as np

timestamps = {
    'prime': {'overhead': [], 'init': []},
    'net': {'overhead': [], 'init': []},
    'hello': {'overhead': [], 'init': []},
    'hash': {'overhead': [], 'init': []},
    'sleep': {'overhead': [], 'init': []}
}

# Open and read the log file
with open('/home/m/Documents/MasterTGN/MasterThesis/results/function_cold_times.log', 'r') as file:
    current_function = None  # Variable to keep track of the current function type

    # Iterate through each line in the log file
    for line in file:
        line = line.strip()  # Remove leading/trailing whitespace

        # Check if the line contains a function type
        if line.startswith('/init'):
            current_function = line.split()[1]  # Extract the function type (prime or net)
        elif line.startswith('timestamp_overhead:'):
            # Extract the overhead timestamp and add it to the respective function's list
            timestamps[current_function]['overhead'].append(int(line.split(':')[1]))
        elif line.startswith('timestamp_init:'):
            # Extract the init timestamp and add it to the respective function's list
            timestamps[current_function]['init'].append(int(line.split(':')[1]))

for function_type, timestamps_dict in timestamps.items():
    print(function_type)
    for timestamp_type, timestamp_list in timestamps_dict.items():
        mean = np.mean(timestamp_list)
        print(timestamp_type, "mean:", mean)

# Print the timestamps for verification
# print("Prime Overhead Timestamps:", prime_overhead_timestamps)
# print("Prime Init Timestamps:", prime_init_timestamps)
# print("Net Overhead Timestamps:", net_overhead_timestamps)
# print("Net Init Timestamps:", net_init_timestamps)
