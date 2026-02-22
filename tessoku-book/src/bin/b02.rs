fn main() {
    proconio::input! {
        a: usize,
        b: usize
    }
    for i in a..=b {
        if 100 % i == 0 {
            println!("Yes");
            break;
        }
        if i == b {
            println!("No");
        }
    }
}
