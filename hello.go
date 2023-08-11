
package main

import "fmt"

// Main function for the action
func Main(obj map[string]interface{}) map[string]interface{} {
	input, ok := obj["input"].(string)
	if !ok {
		input = "stranger"
	}
	fmt.Printf("input=%s\n", input)
	msg := make(map[string]interface{})
	msg["msg"] = "Hello, " + input + "!"
	return msg
}
