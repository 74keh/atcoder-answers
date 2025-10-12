use proconio::{input, marker::Usize1};

fn main() {
    input! {
        _n: usize,
        l: Usize1,
        r: Usize1,
        s: String,
    }
    let yes = !s[l..=r].contains('x');
    println!("{}", if yes { "Yes" } else { "No" });
}
