pub fn is_prime(n: u32) -> bool {
    let max_prime = (n as f64).sqrt() as u32 + 1;
    !(2..max_prime).any(|x| n % x == 0)
}

pub fn nth(n: u32) -> u32 {
    (2..).filter(|&x| is_prime(x)).nth(n as usize).unwrap()
}
