pub struct SquareMatrix<T, const N: usize>(pub [[T; N]; N]);

impl<T, const N: usize> SquareMatrix<T, N> {}
