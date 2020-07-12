pub fn square_of_sum(n: u32) -> u32 {
    let sum: u32 = (1..n + 1).sum();
    let square_of_sum: u32 = sum.pow(2);
    square_of_sum
}

pub fn sum_of_squares(n: u32) -> u32 {
    let sum_of_squares: u32 = (1..n + 1).map(|x| x * x).sum();

    sum_of_squares
}

pub fn difference(n: u32) -> u32 {
    square_of_sum(n) - sum_of_squares(n)
}
