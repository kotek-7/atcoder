use proconio::input;

fn main() {
    input! {
        n:usize, q: usize,
        a_s: [usize; n],
        lr_s: [(usize, usize); q]
    }

    let mut sums = Vec::with_capacity(n);
    sums.push(0);
    let mut sum_temp = 0;
    for a in a_s {
        sum_temp += a;
        sums.push(sum_temp);
    }

    for (l, r) in lr_s {
        println!("{}", sums[r] - sums[l - 1]);
    }
}
