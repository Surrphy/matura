package main

func zad13(n int, A, B []int) int {
	for i := 0; i < n; i++ {
		if zad1(n, i, A, B) {
			return i
		}
	}

	return -1
}
