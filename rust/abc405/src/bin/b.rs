use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [usize; n],
    }

    for i in 0..=n {
        let mut f = vec![false; m+1];
        for &a in &a {
            f[a] = true;
        }

        if let Some(_) = (1..=m).find(|&j| !f[j]) {
            println!("{}", i);
            return;
        }

        a.pop();
    }
}
