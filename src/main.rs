use crate::add_two_numers::Solution as Solution02;
use crate::length_of_longest_substring::Solution as Solution03;
use crate::two_sum::Solution as Solution01;

// 01
mod two_sum;
// 02
mod add_two_numers;
// 03
mod length_of_longest_substring;

fn main() {
    Solution01::two_sum(vec![1, 2, 3], 4);
    Solution02::add_two_numbers(None, None);
    Solution03::length_of_longest_substring(String::from("123"));
}
