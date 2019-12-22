#[allow(dead_code)]

// Rules:
// * It is a six-digit number.
// * The value is within the range given in your puzzle input.
// * Two adjacent digits are the same (like 22 in 122345).
// * Going from left to right, the digits never decrease; they only ever increase or stay the same
//   (like 111123 or 135679).

fn has_adjacent_same(digits: &[u32]) -> bool {
    for i in 1..digits.len() {
        if digits[i - 1] == digits[i] {
            return true;
        }
    }
    false
}

fn digits_decrease(digits: &[u32]) -> bool {
    for i in 1..digits.len() {
        if digits[i - 1] < digits[i] {
            return false;
        }
    }
    true
}

fn is_good(mut num: u32) -> bool {
    // Digits in reverse.
    let mut digits = vec![0; 6];
    for i in 0..6 {
        digits[i] = num % 10;
        num /= 10;
    }
    if !has_adjacent_same(digits.as_slice()) {
        return false;
    }
    digits_decrease(digits.as_slice())
}

fn part1() {
    let mut good = 0;
    for i in 109165..=576723 {
        if is_good(i) {
            good += 1;
        }
    }
    dbg!(good);
}

fn main() {
    part1();
}
