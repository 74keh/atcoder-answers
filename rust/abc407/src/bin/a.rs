use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    }
    println!("{}", (2 * a + b) / (2 * b))
}
