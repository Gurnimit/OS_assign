use std::io::{self, Write};

fn read_line() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input
}

fn fifo(pages: &[i32], nf: usize) {
    let mut frames = vec![-1; nf];
    let mut front = 0;
    let mut faults = 0;

    for &page in pages {
        if !frames.contains(&page) {
            frames[front] = page;
            front = (front + 1) % nf;
            faults += 1;
        }
    }
    println!("FIFO Page Faults  = {}", faults);
}

fn lru(pages: &[i32], nf: usize) {
    let mut frames = vec![-1; nf];
    let mut last_used = vec![0usize; nf];
    let mut filled = 0;
    let mut faults = 0;

    for (i, &page) in pages.iter().enumerate() {
        match frames.iter().position(|&f| f == page) {
            Some(pos) => {
                last_used[pos] = i;
            }
            None => {
                faults += 1;
                if filled < nf {
                    frames[filled] = page;
                    last_used[filled] = i;
                    filled += 1;
                } else {
                    let lru_idx = (0..nf)
                        .min_by_key(|&j| last_used[j])
                        .unwrap_or(0);
                    frames[lru_idx] = page;
                    last_used[lru_idx] = i;
                }
            }
        }
    }
    println!("LRU Page Faults   = {}", faults);
}

fn optimal(pages: &[i32], nf: usize) {
    let mut frames: Vec<i32> = Vec::with_capacity(nf);
    let mut faults = 0;

    for (i, &page) in pages.iter().enumerate() {
        if frames.contains(&page) {
            continue;
        }

        faults += 1;
        if frames.len() < nf {
            frames.push(page);
            continue;
        }

        let mut replace_idx = 0;
        let mut farthest = 0;
        for (j, &frame_page) in frames.iter().enumerate() {
            match pages[i + 1..].iter().position(|&p| p == frame_page) {
                None => {
                    replace_idx = j;
                    break;
                }
                Some(dist) if dist > farthest => {
                    farthest = dist;
                    replace_idx = j;
                }
                _ => {}
            }
        }
        frames[replace_idx] = page;
    }
    println!("Optimal Page Faults = {}", faults);
}

fn main() {
    print!("Enter number of pages in reference string: ");
    io::stdout().flush().unwrap();
    let n: usize = read_line().trim().parse().unwrap();

    print!("Enter the reference string: ");
    io::stdout().flush().unwrap();
    let pages: Vec<i32> = read_line()
        .split_whitespace()
        .take(n)
        .map(|s| s.parse().unwrap())
        .collect();

    print!("Enter number of frames: ");
    io::stdout().flush().unwrap();
    let nf: usize = read_line().trim().parse().unwrap();

    fifo(&pages, nf);
    lru(&pages, nf);
    optimal(&pages, nf);
}
