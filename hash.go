package main

import (
	"fmt"
	"crypto/sha256"
	"encoding/hex"
)

func Main(obj map[string]interface{}) map[string]interface{} {
	input, ok := obj["input"].(string)
	if !ok {
		input = "stranger"
	}
	iterations, ok := obj["iterations"].(int)
	if !ok {
		iterations = 100000
	}

	hash := sha256.New()
	hash.Write([]byte(input))
	hashSum := hash.Sum(nil)

	for i := 0; i < iterations; i++ {
		hash.Write(hashSum)
		hashSum = hash.Sum(nil)
	}
	hashString := hex.EncodeToString(hashSum)
	fmt.Printf("Input: %s\nHash: %s\n", input, hashString)

	msg := make(map[string]interface{})
	msg["msg"] = "hash: " + hashString + "!"
	return msg
}
