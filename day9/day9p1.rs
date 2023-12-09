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

fn extrapolate_value(line: &str) -> i64 {
    let mut nums = line
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect::<Vec<i64>>();

    let mut value = *nums.last().unwrap();

    while !all_equal(&nums) {
        let mut difs: Vec<i64> = Vec::new();

        for i in 1..nums.len() {
            difs.push(nums[i] - nums[i - 1]);
        }

        value += *difs.last().unwrap();
        nums = difs;
    }

    value
}

fn main() {
    let document = fs::read_to_string("day9.in").unwrap();
    let mut sum = 0;
    for line in document.lines() {
        sum += extrapolate_value(line);
    }

    println!("{}", sum);
}
