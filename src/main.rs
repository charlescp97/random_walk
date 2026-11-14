fn walk(steps: u32) -> i64 {
    let mut position: i64 = 0;
    for _ in 0..steps {
        let step = if rand::random::<bool>() { 1 } else { -1 };
        position += step;
    }
    position
}

fn main() {
    for _ in 0..5 {
        println!("Final position after 100 steps: {}", walk(100));
    }
}
