fn main() {
    let input = include_str!("../input");
    let nums: Vec<_> = input
        .lines()
        .map(|l| l.trim().parse::<i64>().unwrap())
        .collect();

    for i in 0..nums.len() {
        for j in i..nums.len() {
            for k in j..nums.len() {
                if nums[i] + nums[j] + nums[k] == 2020 {
                    println!("{}", nums[i] * nums[j] * nums[k]);
                }
            }
        }
    }
}
