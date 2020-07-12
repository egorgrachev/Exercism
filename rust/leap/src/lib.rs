pub fn is_leap_year(year: u64) -> bool {
    if year % 4 > 0 {
        return false;
    }
    // now only divisible by 4 here
    return (year % 400 == 0) || (year % 100 > 0);
}
