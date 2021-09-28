#[cfg(test)]
mod matura_2020_1 {
    use super::*;

    #[test]
    fn returns_true_1_1_2 () {
        let n = 5;
        let k = 2;
        let a: [usize; 5] = [4, 7, 1, 4, 5];
        let b: [usize; 5] = [1, 4, 5, 4, 7];

        let similar_arrays = SimilarArrays::new(n, &a, &b, k);

        assert!(similar_arrays.is_k_similar());
    }

    #[test]
    fn returns_true_2_1_2 () {
        let n = 3;
        let k = 0;
        let a: [usize; 3] = [5, 7, 9];
        let b: [usize; 3] = [5, 7, 9];

        let similar_arrays = SimilarArrays::new(n, &a, &b, k);

        assert!(similar_arrays.is_k_similar());
    }


    #[test]
    fn return_false_1_1_2 () {
        let n = 5;
        let k = 3;
        let a: [usize; 5] = [10, 9, 12, 10, 9];
        let b: [usize; 5] = [10, 10, 9, 9, 12];

        let similar_arrays = SimilarArrays::new(n, &a, &b, k);

        assert!(!similar_arrays.is_k_similar());
    }

    #[test]
    fn return_false_2_1_2 () {
        let n = 5;
        let k = 4;
        let a: [usize; 5] = [3, 6, 5, 1, 8];
        let b: [usize; 5] = [5, 1, 8, 3, 6];

        let similar_arrays = SimilarArrays::new(n, &a, &b, k);

        assert!(!similar_arrays.is_k_similar());
    }

    #[test]
    fn return_true_1_1_3 () {
        let n = 5;
        let k = 2;
        let a: [usize; 5] = [4, 7, 1, 4, 5];
        let b: [usize; 5] = [1, 4, 5, 4, 7];

        let similar_arrays = SimilarArrays::new(n, &a, &b, k);

        assert!(similar_arrays.is_k());
    }

    #[test]
    fn returns_true_2_1_3 () {
        let n = 3;
        let k = 0;
        let a: [usize; 3] = [5, 7, 9];
        let b: [usize; 3] = [5, 7, 9];

        let similar_arrays = SimilarArrays::new(n, &a, &b, k);

        assert!(similar_arrays.is_k());
    }
}

pub struct SimilarArrays<'a> {
    pub n: usize,
    pub k: usize,
    pub b: &'a [usize],
    pub a: &'a [usize],
}

impl SimilarArrays<'_> {
    pub fn new<'a> (n: usize, a: &'a [usize], b: &'a [usize], k: usize) -> SimilarArrays<'a> {
        SimilarArrays {n, a, b, k}
    }

    pub fn is_k_similar(&self) -> bool {
        self.a[0..self.k] == self.b[self.n - self.k..self.n] && self.a[self.k..self.n] == self.b[0..self.n - self.k]
    }

    pub fn is_k(&self) -> bool {
        for k in 0..self.n-1 {
            if SimilarArrays::is_k_similar(&SimilarArrays::new(self.n, self.a, self.b, k)) {
                return true;
            }
        }

        false
    }
}
