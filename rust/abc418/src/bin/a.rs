use proconio::input;

fn main() {
    input! {
        _: usize,
        s: String,
    }
    println!("{}", if s.ends_with("tea") {"Yes"} else {"No"});
}
