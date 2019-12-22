#[allow(dead_code)]

// Rules:
// * It is a six-digit number.
// * The value is within the range given in your puzzle input.
// * Two adjacent digits are the same (like 22 in 122345).
// * Going from left to right, the digits never decrease; they only ever increase or stay the same
//   (like 111123 or 135679).

fn has_adjacent_same(digits: &[u32], adjacent_exactly_two: bool) -> bool {
    let mut count = 1;
    for i in 1..digits.len() {
        if digits[i - 1] == digits[i] {
            count += 1;
            if !adjacent_exactly_two {
                return true;
            }
        } else {
            if count == 2 {
                return true;
            }
            count = 1;
        }
    }
    count == 2
}

fn digits_decrease(digits: &[u32]) -> bool {
    for i in 1..digits.len() {
        if digits[i - 1] < digits[i] {
            return false;
        }
    }
    true
}

fn is_good(mut num: u32, adjacent_exactly_two: bool) -> bool {
    // Digits in reverse.
    let mut digits = vec![0; 6];
    for i in 0..6 {
        digits[i] = num % 10;
        num /= 10;
    }
    if !has_adjacent_same(digits.as_slice(), adjacent_exactly_two) {
        return false;
    }
    digits_decrease(digits.as_slice())
}

fn scan(from: u32, to: u32, adjacent_exactly_two: bool) -> u32 {
    let mut good = 0;
    for i in from..=to {
        if is_good(i, adjacent_exactly_two) {
            good += 1;
        }
    }
    good
}

fn part1(from: u32, to: u32) -> u32 {
    scan(from, to, false)
}

fn part2(from: u32, to: u32) -> u32 {
    scan(from, to, true)
}

fn main() {
    let from = 109165;
    let to = 576723;
    dbg!(part1(from, to));
    dbg!(part2(from, to));
}
