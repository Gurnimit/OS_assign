use std::io::{self, Write};

fn read_line() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input
}

fn print_allocation(title: &str, psize: &[i32], allocation: &[Option<usize>]) {
    println!("\n--- {} ---", title);
    println!("Process No.\tProcess Size\tBlock No.");
    for (i, &size) in psize.iter().enumerate() {
        print!("P{}\t\t{}\t\t", i + 1, size);
        match allocation[i] {
            Some(b) => println!("B{}", b + 1),
            None => println!("Not Allocated"),
        }
    }
}

fn first_fit(bsize: &[i32], psize: &[i32]) {
    let mut block = bsize.to_vec();
    let mut allocation = vec![None; psize.len()];

    for (i, &p) in psize.iter().enumerate() {
        for (j, b) in block.iter_mut().enumerate() {
            if *b >= p {
                allocation[i] = Some(j);
                *b -= p;
                break;
            }
        }
    }
    print_allocation("First Fit", psize, &allocation);
}

fn best_fit(bsize: &[i32], psize: &[i32]) {
    let mut block = bsize.to_vec();
    let mut allocation = vec![None; psize.len()];

    for (i, &p) in psize.iter().enumerate() {
        let mut best_idx = None;
        for (j, &b) in block.iter().enumerate() {
            if b >= p && best_idx.map_or(true, |idx| b < block[idx]) {
                best_idx = Some(j);
            }
        }
        if let Some(idx) = best_idx {
            allocation[i] = Some(idx);
            block[idx] -= p;
        }
    }
    print_allocation("Best Fit", psize, &allocation);
}

fn worst_fit(bsize: &[i32], psize: &[i32]) {
    let mut block = bsize.to_vec();
    let mut allocation = vec![None; psize.len()];

    for (i, &p) in psize.iter().enumerate() {
        let mut worst_idx = None;
        for (j, &b) in block.iter().enumerate() {
            if b >= p && worst_idx.map_or(true, |idx| b > block[idx]) {
                worst_idx = Some(j);
            }
        }
        if let Some(idx) = worst_idx {
            allocation[i] = Some(idx);
            block[idx] -= p;
        }
    }
    print_allocation("Worst Fit", psize, &allocation);
}

fn main() {
    print!("Enter number of memory blocks: ");
    io::stdout().flush().unwrap();
    let bn: usize = read_line().trim().parse().unwrap();

    print!("Enter sizes of blocks: ");
    io::stdout().flush().unwrap();
    let bsize: Vec<i32> = read_line()
        .split_whitespace()
        .take(bn)
        .map(|s| s.parse().unwrap())
        .collect();

    print!("Enter number of processes: ");
    io::stdout().flush().unwrap();
    let pn: usize = read_line().trim().parse().unwrap();

    print!("Enter sizes of processes: ");
    io::stdout().flush().unwrap();
    let psize: Vec<i32> = read_line()
        .split_whitespace()
        .take(pn)
        .map(|s| s.parse().unwrap())
        .collect();

    first_fit(&bsize, &psize);
    best_fit(&bsize, &psize);
    worst_fit(&bsize, &psize);
}
