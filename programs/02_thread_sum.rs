use std::thread;

const NUM_THREADS: usize = 4;

fn main() {
    let n: u64 = 1_000_000;
    let chunk: u64 = n / NUM_THREADS as u64;
    let mut handles = Vec::with_capacity(NUM_THREADS);

    for i in 0..NUM_THREADS {
        let start = i as u64 * chunk;
        let end = if i == NUM_THREADS - 1 {
            n
        } else {
            (i as u64 + 1) * chunk
        };

        let handle = thread::spawn(move || {
            let sum: u64 = (start..end).sum();
            println!("Thread {}: sum[{}..{}) = {}", i + 1, start, end, sum);
            sum
        });

        handles.push(handle);
    }

    let mut total: u64 = 0;
    for handle in handles {
        total += handle.join().expect("Thread panicked");
    }

    println!(
        "Total sum (0 to {}) computed using {} threads = {}",
        n - 1,
        NUM_THREADS,
        total
    );
}
