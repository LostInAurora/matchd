use crate::add_two_numers::Solution as Solution02;
use crate::two_sum::Solution as Solution01;

// 01
mod two_sum;
mod add_two_numers;
// 02
// mod add_two_numers;

fn main() {
    Solution01::two_sum(vec![1, 2, 3], 4);
    Solution02::add_two_numbers(None, None);
}
