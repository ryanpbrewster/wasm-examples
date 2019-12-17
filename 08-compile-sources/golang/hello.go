package main

import "fmt"

func main() {
	fmt.Printf("Hello, World!\n")
	fmt.Printf("2 + 3 + 5 + 7 + 11 + ... + 83 + 89 + 97 == %d\n", sumOfPrimes(100))
}

func sumOfPrimes(hi int) int {
	arr := make([]bool, hi)
	for i := 0; i < hi; i++ {
		arr[i] = true
	}
	for p := 2; p*p < hi; p++ {
		if !arr[p] {
			continue
		}
		for c := p * p; c < hi; c += p {
			arr[c] = false
		}
	}

	sum := 0
	for p := 2; p < hi; p++ {
		if arr[p] {
			sum += p
		}
	}
	return sum
}
