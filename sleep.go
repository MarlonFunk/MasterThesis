package main

import (
	"fmt"
	"time"
)

func Main(params map[string]interface{}) map[string]interface{} {
	duration := time.Duration(params["input"].(float64)) * time.Second
	fmt.Printf("Sleeping for %s...\n", duration)
	time.Sleep(duration)
	fmt.Println("Awake!")

	msg := make(map[string]interface{})
	msg["msg"] = "Sleep and awake completed."
	return msg
}