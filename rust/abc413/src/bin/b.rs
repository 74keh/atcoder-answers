use std::collections::HashSet;
use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [String; n],
    }
    let mut set = HashSet::new();
    for i in 0..n {
        for j in 0..n {
            if i == j {
                continue;
            }
            set.insert(format!("{}{}", s[i], s[j]));
        }
    }
    println!("{}", set.len());
}
