use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        x: usize,
    }
    if a.iter().any(|&a| a == x) {
        println!("Yes");
    } else {
        println!("No");
    }
}
