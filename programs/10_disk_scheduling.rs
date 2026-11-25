use std::io::{self, Write};

fn read_line() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input
}

fn fcfs(req: &[i32], head: i32) {
    let mut total = 0;
    let mut cur = head;
    for &r in req {
        total += (cur - r).abs();
        cur = r;
    }
    println!("FCFS   total head movement = {}", total);
}

fn sstf(req: &[i32], head: i32) {
    let n = req.len();
    let mut visited = vec![false; n];
    let mut total = 0;
    let mut cur = head;

    for _ in 0..n {
        let mut idx = 0;
        let mut best = i32::MAX;
        for i in 0..n {
            let diff = (cur - req[i]).abs();
            if !visited[i] && diff < best {
                best = diff;
                idx = i;
            }
        }
        visited[idx] = true;
        total += best;
        cur = req[idx];
    }
    println!("SSTF   total head movement = {}", total);
}

fn scan(req: &[i32], head: i32, disk_size: i32) {
    let mut arr = req.to_vec();
    arr.push(head);
    arr.sort_unstable();

    let idx = arr.iter().position(|&x| x == head).unwrap();
    let mut total = 0;
    let mut cur = head;

    for &track in &arr[idx + 1..] {
        total += (cur - track).abs();
        cur = track;
    }
    total += (cur - (disk_size - 1)).abs();
    cur = disk_size - 1;

    for &track in arr[..idx].iter().rev() {
        total += (cur - track).abs();
        cur = track;
    }
    println!("SCAN   total head movement = {}", total);
}

fn cscan(req: &[i32], head: i32, disk_size: i32) {
    let mut arr = req.to_vec();
    arr.push(head);
    arr.sort_unstable();

    let idx = arr.iter().position(|&x| x == head).unwrap();
    let mut total = 0;
    let mut cur = head;

    for &track in &arr[idx + 1..] {
        total += (cur - track).abs();
        cur = track;
    }
    total += (cur - (disk_size - 1)).abs();
    total += disk_size - 1;
    cur = 0;

    for &track in &arr[..idx] {
        total += (cur - track).abs();
        cur = track;
    }
    println!("C-SCAN total head movement = {}", total);
}

fn main() {
    print!("Enter number of requests: ");
    io::stdout().flush().unwrap();
    let n: usize = read_line().trim().parse().unwrap();

    print!("Enter the requests: ");
    io::stdout().flush().unwrap();
    let req: Vec<i32> = read_line()
        .split_whitespace()
        .take(n)
        .map(|s| s.parse().unwrap())
        .collect();

    print!("Enter initial head position: ");
    io::stdout().flush().unwrap();
    let head: i32 = read_line().trim().parse().unwrap();

    print!("Enter disk size (max cylinder + 1): ");
    io::stdout().flush().unwrap();
    let disk_size: i32 = read_line().trim().parse().unwrap();

    fcfs(&req, head);
    sstf(&req, head);
    scan(&req, head, disk_size);
    cscan(&req, head, disk_size);
}
