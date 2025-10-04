use proconio::{input, marker::Chars};
use itertools::Itertools;

fn main() {
    input! {
        n: usize,
        m: usize,
        s: [Chars; n],
    }

    let mut scores = vec![0; n];
    for i in 0..m {
        let cnt0 = (0..n).filter(|&j| s[j][i] == '0').count();
        let cnt1 = (0..n).filter(|&j| s[j][i] == '1').count();
        if cnt0 == 0 || cnt1 == 0 {
            for score in scores.iter_mut() {
                *score += 1
            }
        }
        else if cnt0 < cnt1 {
            for j in 0..n {
                if s[j][i] == '0' {
                    scores[j] += 1;
                }
            }
        }
        else {
            for j in 0..n {
                if s[j][i] == '1' {
                    scores[j] += 1;
                }
            }
        }
    }
    let max = *scores.iter().max().unwrap();
    let mut results = vec![];
    for (i, &score) in scores.iter().enumerate() {
        if score == max {
            results.push(i + 1);
        }
    }
    println!("{}", results.iter().join(" "));
}
