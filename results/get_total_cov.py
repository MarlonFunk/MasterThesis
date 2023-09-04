## Script to get CoV of all test results

import re
import numpy as np
import glob
import os

file_list = glob.glob(os.path.join('/home/m/Documents/MasterTGN/MasterThesis/results/final (copy)/CoV', '*cold*'))

# Loop through the list of files and open each one
for file in file_list:
    with open(file, 'r') as file:
        data_str = file.read()
        print(file.name)
        # Split the data into lines and extract the numeric values
        lines = data_str.strip().split('\n')
        data = []
        for line in lines:
            values = line.strip().split(' ')[2]
            # values = re.findall(r'\d+', line)
            # data.extend(map(int, values))
            data.append(int(values))

        # Calculate the mean and standard deviation
        mean = np.mean(data)
        std_dev = np.std(data)

        # Calculate the CoV
        cov = (std_dev / mean) * 100

        print(f"Mean: {mean}")
        print(f"Standard Deviation: {std_dev}")
        print(f"CoV: {cov:.2f}%")