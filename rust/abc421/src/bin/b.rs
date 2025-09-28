use proconio::input;

fn main() {
    input! {
        mut x: usize,
        mut y: usize,
    }
    for _ in 3..=10 {
        (x, y) = (y, f(x + y));
    }
    println!("{}", y);
}

fn f(mut x: usize) -> usize {
    let mut result = 0;
    while x > 0 {
        result = result * 10 + x % 10;
        x /= 10;
    }
    result
}
