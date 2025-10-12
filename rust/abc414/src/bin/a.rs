use proconio::input;

fn main() {
    input! {
        n: usize,
        l: u8,
        r: u8,
    }
    let mut count = 0;
    for _ in 0..n {
        input! {
            x: u8,
            y: u8,
        }
        if x <= l && r <= y {
            count += 1;
        }
    }
    println!("{}", count);
}
