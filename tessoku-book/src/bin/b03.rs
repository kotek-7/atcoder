use proconio::input;

fn main() {
    input! {
        n: usize,
        a_s: [usize; n]
    }

    let mut found = false;
    'outer: for a0 in &a_s {
        for a1 in &a_s {
            for a2 in &a_s {
                if a0 + a1 + a2 == 1000 {
                    found = true;
                    break 'outer;
                }
            }
        }
    }
    
    if found {
        println!("Yes")
    } else {
        println!("No")
    }
}
