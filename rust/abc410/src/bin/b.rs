use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        x: [usize; q],
    }
    let mut b = vec![0; n+1];
    for i in 0..q {
        if i > 0 {
            print!(" ");
        }
        if x[i] >= 1 {
            b[x[i]] += 1;
            print!("{}", x[i]);
        } else {
            let mut y = 1;
            for j in 2..=n {
                if b[y] > b[j] {
                    y = j;
                }
            }
            b[y] += 1;
            print!("{}", y);
        }
    }
    println!();
}
