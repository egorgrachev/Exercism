pub fn raindrops(n: u32) -> String {
    let mut output = String::new();
    let is_factor = |factor: u32| n % factor == 0;

    if is_factor(3) {
        output.push_str("Pling");
    }
    if is_factor(5) {
        output.push_str("Plang");
    }
    if is_factor(7) {
        output.push_str("Plong");
    }

    if output.is_empty() {
        output = n.to_string();
    }

    output
}
