use rand::seq::SliceRandom;

fn main() {
    let mut nums = vec![];
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s.trim_end().to_string();

    let len:u32 = s.trim().parse().unwrap();
    for i in 1..=len { nums.push(i); }

    let mut rng = rand::thread_rng();
    nums.shuffle(&mut rng);

    for i in 0..25 {
        if i == 12 { print!("  *,"); }
        else { print!("{:4}", nums[i]); }
        if i % 5 == 4 { println!(""); }
    }
}
