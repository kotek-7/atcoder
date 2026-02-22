use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize
    }

    let mut count = 0;

    for a in 1..=n {
        for b in 1..=n {
            if k.checked_sub(a + b).is_some_and(|v| v <= n) && k != (a + b) {
                count += 1;
            }
        }
    }

    println!("{}", count);
}
