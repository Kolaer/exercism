pub fn square_of_sum(n: usize) -> usize {
    let tmp = n * (n + 1) / 2;
    tmp * tmp
}

pub fn sum_of_squares(n: usize) -> usize {
    n * (2*n*n + 3*n + 1) / 6
}

pub fn difference(n: usize) -> usize {
    square_of_sum(n) - sum_of_squares(n)
}
