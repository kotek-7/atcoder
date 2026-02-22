use proconio::input;

fn main() {
    input! {
        n: usize
    }
    println!("{:0>10}", f(n));
}

fn f(n: usize) -> usize {
    if n == 0 {
        return 0;
    }
    return f(n / 2) * 10 + n % 2;
}
