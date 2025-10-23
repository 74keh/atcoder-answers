use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        t: Chars,
    }
    let mut yes = true;
    for i in 1..s.len() {
        if s[i].is_uppercase() {
            yes &= t.contains(&s[i - 1]);
        }
    }
    println!("{}", if yes {"Yes"} else {"No"});
}
