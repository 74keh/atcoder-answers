use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }
    for i in 0..s.len() {
        if s[i] == '#' {
            print!("#");
        }
        else if i == 0 || s[i-1] == '#' {
            print!("o");
        }
        else {
            print!(".");
        }
    }
    println!("");
}
