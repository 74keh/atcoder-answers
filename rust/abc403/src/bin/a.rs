use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut sum = 0;
    for i in 0..n {
        if i % 2 == 0 {
            sum += a[i];
        }
    }
    println!("{}", sum);
}
