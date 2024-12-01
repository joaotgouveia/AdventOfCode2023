use std::fs;

fn all_equal(nums: &Vec<i64>) -> bool {
    let i = nums[0];
    for num in nums {
        if i != *num {
            return false;
        }
    }
    true
}

fn extrapolate_value(nums: Vec<i64>) -> i64 {
    if all_equal(&nums) {
        return nums[0];
    }

    let mut difs: Vec<i64> = Vec::new();

    for i in 1..nums.len() {
        difs.push(nums[i] - nums[i - 1]);
    }

    nums.last().unwrap() + extrapolate_value(difs)
}

fn main() {
    let document = fs::read_to_string("day9.in").unwrap();
    let mut sum = 0;
    for line in document.lines() {
        let nums = line
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect::<Vec<i64>>();

        sum += extrapolate_value(nums);
    }

    println!("{}", sum);
}
