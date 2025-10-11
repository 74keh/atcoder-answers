use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }
    let n = s.len();
    let mut result = 0.0_f64;
    if n > 2 {
        for i in 0..(n - 1) {
            if s[i] != 't' {
                continue;
            }
            let mut count = 1;
            for j in (i + 1)..n {
                if s[j] == 't' {
                    count += 1;
                    result = result.max((count - 2) as f64 / (j - i - 1) as f64);
                }
            }
        }
    }
    println!("{}", result);
}
