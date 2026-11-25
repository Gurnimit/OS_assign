use std::io::{self, Write};

const MAX_BLOCKS: usize = 30;

fn read_line() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input
}

fn contiguous_allocation() {
    let mut disk = [false; MAX_BLOCKS];
    println!("\n--- Contiguous Allocation ---");
    print!("Enter number of files: ");
    io::stdout().flush().unwrap();
    let nf: usize = read_line().trim().parse().unwrap();

    for f in 0..nf {
        print!("File {} - starting block, length: ", f + 1);
        io::stdout().flush().unwrap();
        let vals: Vec<usize> = read_line()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        let (start, len) = (vals[0], vals[1]);

        if start + len > MAX_BLOCKS || (start..start + len).any(|i| disk[i]) {
            println!("Allocation failed: blocks not free/out of range.");
            continue;
        }

        for i in start..start + len {
            disk[i] = true;
        }
        println!(
            "File {} allocated blocks {} to {}",
            f + 1,
            start,
            start + len - 1
        );
    }
}

fn linked_allocation() {
    let mut disk = [false; MAX_BLOCKS];
    println!("\n--- Linked Allocation ---");
    print!("Enter number of files: ");
    io::stdout().flush().unwrap();
    let nf: usize = read_line().trim().parse().unwrap();

    for f in 0..nf {
        print!("File {} - number of blocks: ", f + 1);
        io::stdout().flush().unwrap();
        let len: usize = read_line().trim().parse().unwrap();

        print!("File {} allocated blocks: ", f + 1);
        let mut count = 0;
        for i in 0..MAX_BLOCKS {
            if count == len {
                break;
            }
            if !disk[i] {
                disk[i] = true;
                if count > 0 {
                    print!("-> ");
                }
                print!("{} ", i);
                count += 1;
            }
        }
        println!("(NULL)");
        if count < len {
            println!("Warning: disk full, could not allocate all blocks.");
        }
    }
}

fn indexed_allocation() {
    let mut disk = [false; MAX_BLOCKS];
    println!("\n--- Indexed Allocation ---");
    print!("Enter number of files: ");
    io::stdout().flush().unwrap();
    let nf: usize = read_line().trim().parse().unwrap();

    for f in 0..nf {
        print!("File {} - number of data blocks needed: ", f + 1);
        io::stdout().flush().unwrap();
        let len: usize = read_line().trim().parse().unwrap();

        let index_block = match disk.iter().position(|&used| !used) {
            Some(idx) => {
                disk[idx] = true;
                idx as isize
            }
            None => -1,
        };

        print!("File {} - Index block: {}, Data blocks: ", f + 1, index_block);
        let mut count = 0;
        for i in 0..MAX_BLOCKS {
            if count == len {
                break;
            }
            if !disk[i] {
                disk[i] = true;
                print!("{} ", i);
                count += 1;
            }
        }
        println!();
    }
}

fn main() {
    println!("Menu:\n1. Contiguous Allocation\n2. Linked Allocation");
    println!("3. Indexed Allocation");
    print!("Enter choice: ");
    io::stdout().flush().unwrap();
    let choice: i32 = read_line().trim().parse().unwrap();

    match choice {
        1 => contiguous_allocation(),
        2 => linked_allocation(),
        3 => indexed_allocation(),
        _ => println!("Invalid choice"),
    }
}
