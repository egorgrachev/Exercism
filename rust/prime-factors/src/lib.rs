pub fn factors(mut n: u64) -> Vec<u64> {
    let mut output = Vec::<u64>::new();
    let mut candidates = 2..;

    while n > 1 {
        let i = candidates.next().unwrap();

        while n % i == 0 {
            output.push(i);
            n = n / i;
        }
    }
    
    output
}
