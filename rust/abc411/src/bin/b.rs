use proconio::input;

fn main() {
    input! {
        n: usize,
        d: [usize; n-1],
    }
    for i in 0..n-1 {
        let mut a = d[i];
        print!("{}", a);
        for j in i+1..n-1 {
            a += d[j];
            print!(" {}", a);
        }
        println!();
    }
}
