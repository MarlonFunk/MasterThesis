library(ggplot2)
library(dplyr)
library(tidyr)
library(pals)


args <- commandArgs(trailingOnly = TRUE)
file_path <- args[1]
#data_lines <- readLines(file_path)
data_lines <- readLines("/home/m/MasterThesis/results/final/2_wasm_concurrency_hash.log")
#data_lines <- readLines("/home/m/MasterThesis/results/2_wasm_concurrency_hello.log")

# Remove the leading comma and split the values
data_list <- lapply(data_lines, function(line) {
  values <- unlist(strsplit(line, ",\\s*"))
  as.numeric(values[values != ""])
})

# Add an index column to the data list
data_df <- data.frame(Value = unlist(data_list), 
                      Index = rep(seq_along(data_list), lengths(data_list)))

highest_outlier <- aggregate(Value ~ Index, data = data_df, FUN = max)# Creating the boxplot using ggplot

ggplot(data_df, aes(x = factor(Index), y = Value)) +
  #geom_violin() +
  geom_boxplot(,width=0.2, outlier.alpha = 0.55) +
  #geom_boxplot(width=0.5, aes(fill=factor(Index)), show.legend = F, outlier.alpha = 0.55) +
  #geom_boxplot(width=0.5) +
  geom_point(stat="summary", fun="mean", shape=15, color="navy")+
  geom_text(stat = "summary", fun = "mean", aes(label = sprintf("%.2f", ..y..), fontface = "bold"),
            vjust = 2.9, color = "navy", size = 3.5) +
  geom_text(data = highest_outlier, aes(label = sprintf("%.2f", Value), x = factor(Index), 
                                       fontface = "bold", y = Value + 3),  size = 3.5) +
  labs(title = "WaitTime of wasm modules",
       x = "Number of Responses",
       y = "WaitTime time in ms") +
  #scale_fill_manual(values=as.vector(polychrome(15)))+
  theme_minimal()

ggsave("concurrency.pdf")