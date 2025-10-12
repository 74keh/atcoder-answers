use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }
    let mut a = vec![];
    for i in 0..s.len() {
        if s[i] == '#' {
            a.push(i+1);
        }
    }
    for i in (0..a.len()).step_by(2) {
        println!("{},{}", a[i], a[i+1]);
    }
}
