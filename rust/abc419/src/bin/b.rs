use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::input;

fn main() {
    input! {
        q: usize,
    }
    let mut queue = BinaryHeap::new();
    for _ in 0..q {
        input! {
            t: u8,
        }
        match t {
            1 => {
                input! {
                    x: usize,
                }
                queue.push(Reverse(x));
            }
            2 => {
                let Some(Reverse(result)) = queue.pop() else {
                    unreachable!()
                };
                println!("{}", result);
            }
            _ => unreachable!(),
        }
    }
}
