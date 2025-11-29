use proconio::marker::Chars;
use proconio::input;

fn main() {
    input! {
        t: Chars,
        u: Chars,
    }

    for i in 0..=t.len() - u.len() {
        let mut ok = true;

        for j in 0..u.len() {
            if t[i + j] != u[j] && t[i + j] != '?' {
                ok = false;
                break;
            }
        }

        if ok {
            println!("Yes");
            return;
        }

    }

    println!("No");
}
