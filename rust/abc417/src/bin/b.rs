use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [usize; n],
        b: [usize; m],
    }
    for b in b {
        for i in 0..a.len() {
            if a[i] == b {
                a.remove(i);
                break;
            }
        }
    }
    if !a.is_empty() {
        println!("{}", a.iter().join(" "));
    }
}
