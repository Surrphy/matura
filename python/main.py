import unittest

def czy_k_podobne(n, A, B, k):
    return A[0:k] == B[n-k:] and A[k:] == B[:n-k]

def czy_podobne(n, A, B):
    for k in range(n):
        if czy_k_podobne(n, A, B, k):
            return True
    return False

class Exercise1Tests(unittest.TestCase):
    def test1(self):
        self.assertEqual(czy_k_podobne(3, [5, 7, 9], [5, 7, 9], 0), True)

    def test2(self):
        self.assertEqual(czy_k_podobne(5, [4,7,1,4,5], [1,4,5,4,7], 2), True)

    def test3(self):
        self.assertEqual(czy_k_podobne(5, [10, 9, 12, 10, 9], [10, 10, 9, 9, 12], 3), False)

    def test4(self):
        self.assertEqual(czy_k_podobne(5, [3, 6, 5, 1, 8], [5, 1, 8, 3, 6], 4), False)

    def test5(self):
        self.assertEqual(czy_k_podobne(5, [1, 2, 3, 4, 5], [3, 4, 5, 1, 2], 2), True)

    def test6(self):
        self.assertEqual(czy_k_podobne(9, [1,1,1,1,3,1,1,1,1], [3,1,1,1,1,1,1,1,1], 4), True)

    def test7(self):
        self.assertEqual(czy_k_podobne(56, [4, 2, 4, 4, 2, 6], [4, 4, 2, 6, 4, 2], 1), False)

class Exercise2Tests(unittest.TestCase):
    def test1(self):
        self.assertEqual(czy_podobne(5, [3, 6, 5, 1, 8], [5, 1, 8, 3, 6]), True)

    def test2(self):
        self.assertEqual(czy_podobne(3, [1,2,3], [4,5,6]), False)

    def test3(self):
        self.assertEqual(czy_podobne(4, [1,2,3,4], [4,5,6,7]), False)

    def test4(self):
        self.assertEqual(czy_podobne(9, [8, 9, 4, 21, 5, 1, 1, 1, 2], [1, 1, 1, 2, 8, 9, 4, 21, 5]), True)

if __name__ == "__main__":
    unittest.main()