package main

import (
	"fmt"
	"testing"
)

type args struct {
	n int
	k int
	A []int
	B []int
}

func Test_zad1(t *testing.T) {
	tests := []struct {
		args args
		want bool
	}{
		{
			args {
				3, 0, []int{5, 7, 9}, []int{5, 7, 9},
			},
			true,
		},
		{
			args {
				5, 2, []int{4, 7, 1, 4, 5}, []int{1, 4, 5, 4, 7},
			},
			true,
		},
		{
			args {
				5, 3, []int{10, 9, 12, 10, 9}, []int{10, 10, 9, 9, 12},
			},
			false,
		},
		{
			args {
				5, 4, []int{3, 6, 5, 1, 8}, []int{5, 1, 8, 3, 6},
			},
			false,
		},
	}

	for index, tt := range tests {
		t.Run(fmt.Sprintf("Test #%d", index), func(t *testing.T) {
			if got := zad1(tt.args.n, tt.args.k, tt.args.A, tt.args.B); got != tt.want {
				t.Errorf("zad1(%+v) = %v, want %v", tt.args, got, tt.want)
			}

			if tt.want {
				if zad13(tt.args.n, tt.args.A, tt.args.B) != tt.args.k {
					t.Errorf("zad13(%+v) not equals {tt.args.k}", tt.args)
				}
			}
		})
	}
}
