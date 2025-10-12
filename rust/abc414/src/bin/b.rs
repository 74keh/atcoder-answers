use proconio::input;

fn main() {
    input! {
        n: usize,
        cl: [(char, usize); n]
    }
    let mut result = String::new();
    for (c, l) in cl {
        if result.len() + l > 100 {
            println!("Too Long");
            return;
        }
        for _ in 0..l {
            result.push(c);
        }
    }
    println!("{}", result);
}
