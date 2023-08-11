
package main

import (
	"fmt"
	"time"
)

// isPrime checks if a number is prime
func isPrime(n int) bool {
	if n <= 1 {
		return false
	}
	if n <= 3 {
		return true
	}
	if n%2 == 0 || n%3 == 0 {
		return false
	}
	i := 5
	for i*i <= n {
		if n%i == 0 || n%(i+2) == 0 {
			return false
		}
		i += 6
	}
	return true
}
// Main function for the action
func Main(obj map[string]interface{}) map[string]interface{} {
	startTime := time.Now()

	var p int
	count := 0
	i := 2
	          //5000000
	for count < 5000 {
		if isPrime(i) {
			p = i
			count++
		}
		i++
	}
	fmt.Printf("Prime number at index 20000001: %d\n", p)

	elapsed := time.Since(startTime)
	msg := make(map[string]interface{})

	msg["msg"] = fmt.Sprintf("prime: %d. calc_time: %v", p, elapsed)
	return msg
}
