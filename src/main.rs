use std::fs::File;
use std::io::{self, Write};

// random step with equal probablity
fn step() -> i64 {
    if rand::random::<bool>() { 1 } else { -1 }
}

//MSD at each step 0 .. steps averaged over all walkers.
fn compute_msd(walkers: usize, steps: usize) -> Vec<f64> {
    let mut sum_sq = vec![0.0_f64; steps + 1];

    for _ in 0..walkers {
        let mut x: i64 = 0;
        for n in 1..=steps {
            x += step();
            sum_sq[n] += (x * x) as f64;
        }
    }
    sum_sq.iter().map(|s| s / walkers as f64).collect()
}

//write MSD to csv
fn write_csv(path: &str, msd: &[f64]) -> io::Result<()> {
    let mut file = File::create(path)?;

    writeln!(file, "step,msd")?;

    for (n, value) in msd.iter().enumerate() {
        writeln!(file, "{}, {}", n, value)?;
    }
    Ok(())
}

fn main() -> io::Result<()> {
    let walkers = 10_000;
    let steps = 1_000;

    let msd = compute_msd(walkers, steps);
    write_csv("msd.csv", &msd)?;
    println!("Final MSD: {:.1} (theory: 1000)", msd[1_000]);
    Ok(())
}
