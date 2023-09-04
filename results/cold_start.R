## Script to create plots for cold start test

library(ggplot2)
library(dplyr)
library(pals)
library(stringr)

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

args <- commandArgs(trailingOnly = TRUE)
file_path <- args[1]
file_name <- str_replace(basename(file_path),'.log', '')
#print(file_name)
#data <- readLines(file_path)
data <- readLines("/home/m/MasterThesis/results/final/2_docker_cold_start.log")
#data <- readLines("/home/m/MasterThesis/results/2_docker_cold_start.log")

lines <- read.table(text = data, header = FALSE)
colnames(lines) <- c("activationID", "responses", "time")
lines_sorted <- arrange(lines, responses)
df_lines_sorted <- data.frame(lines_sorted)

result_cv <- aggregate(time ~ responses, data = df_lines_sorted, FUN = calculate_cv)
result_sd = aggregate(time ~ responses, data = df_lines_sorted, FUN = calculate_sd)
statistics_table <- merge(result_cv, result_sd, by = "responses", all = TRUE)
result_mean <- aggregate(time ~ responses, data = df_lines_sorted, FUN = mean)
result_median <- aggregate(time ~ responses, data = df_lines_sorted, FUN =median)
statistics_table <- merge(result_cv, result_sd, by = "responses", all = TRUE)
statistics_table <- merge(statistics_table, result_mean, by = "responses", all = TRUE)
statistics_table <- merge(statistics_table, result_median, by = "responses", all = TRUE)
colnames(statistics_table) <- c("Index", "Coefficient_of_Variation", "Standard_Deviation", "Mean", "Median")
statistics_table$Index <- NULL
print(statistics_table)

highest_outlier <- aggregate(time ~ responses, data = lines_sorted, FUN = max)# Creating the boxplot using ggplot

ggplot(data = df_lines_sorted, aes(x = as.factor(responses), y = time, group = responses)) +
  #geom_violin() +
  #geom_boxplot(width=0.5, aes(fill=factor(responses)), show.legend = F) +
  geom_boxplot(width=0.5, outlier.alpha = 0.55, fill="aquamarine", show.legend = F) +
  geom_point(stat="summary", fun="mean", shape=15, color="#ff2525")+
  #geom_text(stat = "summary", fun = "mean", aes(label = sprintf("%.2f", ..y..), fontface = "bold"),
  #          vjust = 2, color = "navy", size = 3.5) +
  geom_text(data = highest_outlier, aes(label = sprintf("%d", time), x = factor(responses), 
                                       fontface = "bold", vjust=-1), color = "black", size = 3.5) +
  #geom_point(data=statistics_table, aes(x=responses, y=Coefficient_of_Variation))+
  #geom_line(data=statistics_table, aes(x=as.factor(responses), y=Coefficient_of_Variation))+
  labs(x = "Concurrent number of Activations",
       y = "Cold start time in ms") +
  #scale_fill_manual(values=as.vector(polychrome(15)))+
  theme_minimal()
ggsave(paste(file_name, ".pdf", sep = ""))