
package main

import (
	"fmt"
	"time"
)

// Main function for the action
func Main(obj map[string]interface{}) map[string]interface{} {
	requestTime := time.Now()
	time.Sleep(300 * time.Millisecond)
	// elapsed := time.Since(requestTime)
	// fmt.Printf("Time elapsed: %v\n", elapsed)
	msg := make(map[string]interface{})
	msg["msg"] = fmt.Sprintf("request_time: %s", requestTime)
	return msg
}
