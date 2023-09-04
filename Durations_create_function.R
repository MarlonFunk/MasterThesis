## Script to create stacked bar plots with mean durations

library(ggplot2)

data <- data.frame(
  Task = rep(c("sleep", "hash", "hello", "net", "prime"), each = 2),
  Type = rep(c("Type I: providing Wasi", "Type II: Instantiating module"), times = 5),
  Mean = c(146.2, 175.5, 126.2, 141.1, 172, 177.8, 149.1, 149.4, 134.6, 163.2)
)

data <- data.frame(
  Task = rep(c("sleep", "hash", "hello", "net", "prime"), each = 2),
  Type = rep(c("Type I: overhead", "Type II: init"), times = 5),
  Mean = c(389.5, 893.0, 453.5, 1008.9, 499.4,1176.9, 374.8, 920.4, 548.6, 1261.6)
)


# Create a grouped bar plot
ggplot(data, aes(x = Task, y = Mean, fill = Type)) +
  geom_bar(stat = "identity", position = position_dodge(width = 0.8)) +
  labs(x = "",
       y = "Mean Value in us",
       fill = "Type") +
  theme_minimal()