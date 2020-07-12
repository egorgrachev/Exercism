pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let is_multiple = |x: u32| factors.into_iter().any(|&i| i != 0 && x % i == 0);
    let result = (0..limit).filter(|&x| is_multiple(x)).sum();
    result
}
