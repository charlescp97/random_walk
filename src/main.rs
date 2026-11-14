fn walk(steps: u32) -> i64 {
    let mut position: i64 = 0;
    for _ in 0..steps {
        //use turbofish == let step: bool = rand::random()
        let step = if rand::random::<bool>() { 1 } else { -1 };
        position += step;
    }
    position
}

fn main() {
    let walkers = 10_000;
    let steps = 1_000;

    let finals: Vec<i64> = (0..walkers).map(|_| walk(steps)).collect();

    let mean: f64 = finals.iter().sum::<i64>() as f64 / walkers as f64;
    let msd: f64 = finals.iter().map(|&x| (x * x) as f64).sum::<f64>() / walkers as f64;

    println!("Walkers: {}, steps: {}", walkers, steps);
    println!("Mean positions: {:.3} (theory: 0)", mean);
    println!("Mean Squared Displacement: {:.1} (theory: {})", msd, steps);
    println!(
        "RMS distance: {:.2} (theory: {}:.2)",
        msd.sqrt(),
        (steps as f64).sqrt()
    );
}
