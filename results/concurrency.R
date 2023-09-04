## Script to create plots for concurrency tests

library(ggplot2)
library(dplyr)
library(tidyr)
library(pals)
library(stringr)
library(ggrepel)


calculate_cv <- function(x) {
  if (length(x) <= 1 || all(is.na(x))) {
    return(c(NA, NA))
  } else {
    sd_value <- sd(x, na.rm = TRUE)
    mean_value <- mean(x, na.rm = TRUE)
    cv_value <- (sd_value / mean_value) * 100
    return(cv_value)
  }
}

calculate_sd <- function(x) {
  sd(x, na.rm = TRUE)
}

remove_nas <- function(x) {
  if (is.list(x)) {
    return(lapply(x, remove_nas))
  } else {
    return(na.omit(x))
  }
}

args <- commandArgs(trailingOnly = TRUE)
file_path <- args[1]
file_name <- str_replace(basename(file_path),'.log', '')


#data_lines <- readLines(file_path)

data_lines <- readLines("/home/m/MasterThesis/results/final/4_wasm_concurrency_mixed_9_1.log")
#data_lines <- readLines("/home/m/MasterThesis/results/2_docker_concurrency_hash.log")

# Remove the leading comma and split the values
data_list <- lapply(data_lines, function(line) {
  values <- unlist(strsplit(line, ",\\s*"))
  as.numeric(values[values != ""])
})
data_list <- data_list[-16]


# Add an index column to the data list
data_df <- data.frame(Value = unlist(data_list), 
                      Index = rep(seq_along(data_list), lengths(data_list)))

# Calculate standard deviation and CV for each index
result_cv <- aggregate(Value ~ Index, data = data_df, FUN = calculate_cv)
result_sd <- aggregate(Value ~ Index, data = data_df, FUN = calculate_sd)
result_mean <- aggregate(Value ~ Index, data = data_df, FUN = mean)
#result_median <- aggregate(Value ~ Index, data = data_df, FUN = median)
statistics_table <- merge(result_cv, result_sd, by = "Index", all = TRUE)
statistics_table <- merge(statistics_table, result_mean, by = "Index", all = TRUE)
#statistics_table <- merge(statistics_table, result_median, by = "Index", all = TRUE)
colnames(statistics_table) <- c("Index", "Coefficient_of_Variation", "Standard_Deviation", "Mean")
statistics_table$Index <- NULL
print(statistics_table)


df <- data.frame(
  Coefficient_of_Variation = result_cv$Value,
  Mean = result_mean$Value
)

file_name = "4_wasm_concurrency_mixed_9_1.csv"

write.csv(df, file = paste("csv_", file_name, sep = ""), row.names = FALSE)

highest_outlier <- aggregate(Value ~ Index, data = data_df, FUN = max)# Creating the boxplot using ggplot
# 
# ggplot(data_df, aes(x = factor(Index), y = Value)) +
#   #geom_violin() +
#   geom_boxplot(width=0.5, outlier.alpha = 0.55, fill="aquamarine") + #TODO: Color or not?
#   #geom_boxplot(width=0.5, aes(fill=factor(Index)), show.legend = F, outlier.alpha = 0.55) +
#   #geom_boxplot(width=0.5) +
#   geom_point(stat="summary", fun="mean", shape=15, color="#ff2525")+
#   #geom_text(stat = "summary", fun = "mean", aes(label = sprintf("%.2f", ..y..), fontface = "bold"),
#   #          vjust = 2.9, color = "navy", size = 3.5) +
#   geom_text(data = highest_outlier, aes(label = sprintf("%.d", Value), x = factor(Index), 
#                                         fontface = "bold", vjust=-1),  size = 3.5) +
#   #labs(title = "WaitTime of Wasm modules",
#   labs(x = "Concurrent number of Activations",
#        y = "Warm latency (ms)") +
#   #scale_fill_manual(values=as.vector(polychrome(15)))+
#   theme_minimal()
# 
# ggsave(paste(file_name, ".pdf", sep = ""))