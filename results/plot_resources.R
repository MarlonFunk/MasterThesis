## Script to create plots of measured resources

library(ggplot2)
library(dplyr)
library(gridExtra)
library(cowplot)
library(stringr)
library(ggrepel)


theme_set(theme_minimal())
# Read the log file into a data frame
data_list <- read.table("/home/m/MasterThesis/results/final/docker_res.log", header = TRUE)
data <- read.table("/home/m/MasterThesis/results/final/wasm_res.log", header = TRUE)

file_name <- str_replace(basename(file_path),'_res.log', '')


## Docker
memory_numeric <- as.numeric(gsub("MiB", "", data_list[[4]]))

outlier_indices_cpu <- c(22, 25, 31, 36, 44 ,50)

df <- data.frame(Observations = seq_along(data_list[[1]]), CPU = data_list[[2]], Memory = memory_numeric)

cpu_plot <- ggplot(df, aes(x = Observations, y = CPU)) +
  geom_line() +
  geom_text_repel(data = df[outlier_indices_cpu, ], aes(label = CPU),hjust = 1, size = 3.5)+
  labs(x = "Observations", y = "CPU%")
ggsave(paste("docker_usage_cpu.pdf", sep=''))



outlier_indices_mem <- c(6, 9, 14, 18, 27, 31, 35, 38, 44, 50)
memory_plot <- ggplot(df, aes(x = Observations, y = Memory)) +
  geom_line() +
  geom_text_repel(data = df[outlier_indices_mem, ], aes(label = Memory),hjust = 1, size = 3.5)+
  labs(x = "Observations", y = "Memory (MiB)")
ggsave(paste("docker_usage_mem.pdf", sep=''))

#ggsave(paste(filename,"_usage_memory.pdf", sep=''))


##Wasm
cpu_usage = ggplot(data, aes(x = seq_along(X.CPU), y=X.CPU))+
  geom_line()+
  geom_text(aes(x = which.max(X.CPU)-10, y = max(X.CPU), label = max(X.CPU)), 
                vjust = -0.5, hjust = -0.5, size = 3.5)+
  labs(x = "Observations", y = "Percentage of CPU Usage")
ggsave(paste("wasm_usage_cpu.pdf", sep=''))



data$RSS_MiB <- data$RSS / 1024
#max_rss <- data[which.max(data$RSS_MiB]
rss_usage = ggplot(data, aes(x = seq_along(RSS_MiB), y=RSS_MiB))+
  geom_line()+
  geom_text(aes(x = which.max(RSS_MiB)-30, y = max(RSS_MiB), label = sprintf("%.2f", max(RSS_MiB))), 
                vjust = -0.5, hjust = -0.5, size = 3.5)+
  labs(x = "Observations", y = "Memory (MiB)")
ggsave(paste("wasm_usage_mem.pdf", sep=''))