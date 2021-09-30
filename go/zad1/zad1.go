package main

import "reflect"

func zad1(n, k int, A, B []int) bool {
	if k < 0 || n <= k {
		return false
	}

	return reflect.DeepEqual(A[:k], B[n-k:n]) && reflect.DeepEqual(A[k:n], B[:n-k])
}
