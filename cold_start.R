library(ggplot2)
library(dplyr)
library(pals)

args <- commandArgs(trailingOnly = TRUE)
file_path <- args[1]

#data <- readLines(file_path)
data <- readLines("/home/m/MasterThesis/results/final/2_wasm_cold_start.log")
#data <- readLines("/home/m/MasterThesis/results/2_docker_cold_start.log")

lines <- read.table(text = data, header = FALSE)
colnames(lines) <- c("activationID", "responses", "time")
lines_sorted <- arrange(lines, responses)

#TODO: Color or some stuff?
#TODO: After 10 tests, violin plot ontop may be useful too?

highest_outlier <- aggregate(time ~ responses, data = lines_sorted, FUN = max)# Creating the boxplot using ggplot

ggplot(data = lines_sorted, aes(x = as.factor(responses), y = time, group = responses)) +
  #geom_violin() +
  geom_boxplot(width=0.5, aes(fill=factor(responses)), show.legend = F) +
  geom_point(stat="summary", fun="mean", shape=15, color="navy")+
  geom_text(stat = "summary", fun = "mean", aes(label = sprintf("%.2f", ..y..), fontface = "bold"),
            vjust = 2, color = "navy", size = 3.5) +
  geom_text(data = highest_outlier, aes(label = sprintf("%.2f", time), x = factor(responses), 
                                       fontface = "bold", y = time + 10), color = "black", size = 3.5) +
  labs(title = "Cold start time of wasm modules",
       x = "Number of Responses",
       y = "Cold start time in ms") +
  #scale_fill_manual(values=as.vector(polychrome(15)))+
  theme_minimal()
ggsave("cold_start.pdf")