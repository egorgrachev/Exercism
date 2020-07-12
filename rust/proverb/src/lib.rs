pub fn build_proverb(list: &[&str]) -> String {
    if list.is_empty() {
        return String::new();
    }

    let mut output_vec = list
        .windows(2)
        .map(|x| format!("For want of a {0} the {1} was lost.", x[0], x[1]))
        .collect::<Vec<_>>();
    output_vec.push(format!("And all for the want of a {}.", list[0]));
    output_vec.join("\n")
}
