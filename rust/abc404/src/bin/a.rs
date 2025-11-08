use proconio::input;

fn main() {
    input! {
        s: String,
    }
    if let Some(result) = ('a'..='z').find(|&c| !s.contains(c)) {
        println!("{}", result);
    } else {
        println!("None");
    }
}
