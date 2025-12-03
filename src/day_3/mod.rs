
fn joltage(s: &str, n: usize) -> u64 {
    let digits: Vec<u8> = s.chars().map(|ch| ch.to_digit(10).unwrap() as u8).collect();
    let num_digits = digits.len();

    let mut res = 0;
    let mut current_pos = 0;

    for rem in (0..n).rev() {
        let end = num_digits  - 1 - rem;

        let mut max_digit = 0;
        let mut max_idx = 0;

        let available_digits = &digits[current_pos..=end];

        for (idx, &digit) in available_digits.iter().enumerate() {
            if digit > max_digit {
                max_digit = digit;
                max_idx = current_pos + idx;
                if max_digit == 9 {
                    break;
                }
            }
        }

        res = res * 10 + max_digit as u64;
        current_pos = max_idx + 1;
    }

    res
}

pub fn part_1(vec: &Vec<String>) -> u64 {
    vec.iter().map(|f| joltage(&f, 2)).sum()
}

pub fn part_2(vec: &Vec<String>) -> u64 {
    vec.iter().map(|f| joltage(&f, 12)).sum()
}
