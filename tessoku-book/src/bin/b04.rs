use proconio::input;

fn main() {
    input! {n: usize}
    let digits = n
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<_>>();

    let mut digit = digits.len();
    let ans = digits.into_iter().fold(0, |prev, d| {
        digit -= 1;
        return prev + d * 2_u32.pow(digit as u32);
    });

    println!("{}", ans);
}
