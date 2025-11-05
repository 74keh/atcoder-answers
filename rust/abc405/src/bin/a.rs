use proconio::input;

fn main() {
    input! {
        r: u32,
        x: u32
    }
    let rated = if x == 1 {
        1600 <= r && r <= 2999
    } else {
        1200 <= r && r <= 2399
    };
    println!("{}", if rated {"Yes"} else {"No"});
}
