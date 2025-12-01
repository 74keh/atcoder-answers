use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [usize; n],
    }

    loop {
        let mut f = vec![false; m];
        for &a in &a {
            f[a] = true;
        }

        if let Some(i) = (1..=m).find(|&i| !f[i]) {
            println!("{}", i);
            return;
        }
        
        a.pop();
    }
}
