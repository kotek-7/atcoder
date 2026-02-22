fn main() {
    proconio::input! {
        n: usize,
        x: usize,
        a_s: [usize; n]
    }

    if a_s.into_iter().any(|a| a == x) {
        println!("Yes")
    } else {
        println!("No")
    }
}
