use crate::experimental::matrix::*;

#[test]
fn test() {
    let a = Matrix::from([[1, 2], [3, 4]]);
    let b = Matrix::from([[3, 1], [4, 1]]);

    assert_eq!(*a[0], [1, 2]);
    assert_eq!(*a[1], [3, 4]);

    assert_eq!(&a + &b, Matrix::from([[4, 3], [7, 5]]));
    assert_eq!(&a * &b, Matrix::from([[11, 3], [25, 7]]));
}
