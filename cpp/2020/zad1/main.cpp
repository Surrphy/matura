#include <iostream>
#include <vector>
#include "../../utils.h"

using namespace std;

vector<int> slicing(vector<int>& arr, int X, int Y) {
    auto start = arr.begin() + X;
    auto end = arr.begin() + Y + 1;
 
    vector<int> result(Y - X + 1);
 
    copy(start, end, result.begin());
 
    return result;
}

bool czy_k_podobne(int n, vector<int> A, vector<int> B, int k) {
    if ((slicing(A, 0, k - 1) == slicing(B, n - k, n - 1)) && (slicing(A, k, n - 1) == slicing(B, 0, n - k - 1)))
        return true;
    
    return false;
}

bool czy_podobne(int n, vector<int> A, vector<int> B) {
    for (int i = 0; i < n; i++)
        if (czy_k_podobne(n, A, B, i))
            return true;

    return false;
}

int main() {
    make_test(czy_k_podobne(3, {5, 7, 9}, {5, 7, 9}, 0), true);
    make_test(czy_k_podobne(5, {4,7,1,4,5}, {1,4,5,4,7}, 2), true);
    make_test(czy_k_podobne(5, {10, 9, 12, 10, 9}, {10, 10, 9, 9, 12}, 3), false);
    make_test(czy_k_podobne(5, {3, 6, 5, 1, 8}, {5, 1, 8, 3, 6}, 4), false);
    make_test(czy_k_podobne(5, {1, 2, 3, 4, 5}, {3, 4, 5, 1, 2}, 2), true);
    make_test(czy_k_podobne(9, {1,1,1,1,3,1,1,1,1}, {3,1,1,1,1,1,1,1,1}, 4), true);
    make_test(czy_k_podobne(56, {4, 2, 4, 4, 2, 6}, {4, 4, 2, 6, 4, 2}, 1), false);

    make_test(czy_podobne(5, {3, 6, 5, 1, 8}, {5, 1, 8, 3, 6}), true);
    make_test(czy_podobne(3, {1,2,3}, {4,5,6}), false);
    make_test(czy_podobne(4, {1,2,3,4}, {4,5,6,7}), false);
    make_test(czy_podobne(9, {8, 9, 4, 21, 5, 1, 1, 1, 2}, {1, 1, 1, 2, 8, 9, 4, 21, 5}), true);

    return 0;
}