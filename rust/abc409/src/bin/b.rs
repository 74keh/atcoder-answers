use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    }
    a.sort_unstable();
    let mut ans = 0;
    for i in 0..n {
        ans = ans.max(a[i].min(a.len() - i));
        
    }
    println!("{}", ans);
}
