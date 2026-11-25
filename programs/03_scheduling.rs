use std::collections::VecDeque;
use std::io::{self, Write};

#[derive(Clone)]
struct Process {
    pid: usize,
    at: i32,
    bt: i32,
    pr: i32,
    ct: i32,
    tat: i32,
    wt: i32,
    rem: i32,
}

fn read_line() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input
}

fn print_result(procs: &[Process]) {
    let n = procs.len() as f64;
    let mut total_wt = 0.0;
    let mut total_tat = 0.0;

    println!("\nPID\tAT\tBT\tCT\tTAT\tWT");
    for p in procs {
        println!(
            "{}\t{}\t{}\t{}\t{}\t{}",
            p.pid, p.at, p.bt, p.ct, p.tat, p.wt
        );
        total_wt += p.wt as f64;
        total_tat += p.tat as f64;
    }
    println!("Average Waiting Time    = {:.2}", total_wt / n);
    println!("Average Turnaround Time = {:.2}", total_tat / n);
}

fn fcfs(procs: &mut [Process]) {
    let mut time = 0;
    for p in procs.iter_mut() {
        if time < p.at {
            time = p.at;
        }
        time += p.bt;
        p.ct = time;
        p.tat = p.ct - p.at;
        p.wt = p.tat - p.bt;
    }
    print_result(procs);
}

fn sjf(procs: &mut [Process]) {
    let n = procs.len();
    let mut completed = 0;
    let mut time = 0;
    let mut done = vec![false; n];

    while completed < n {
        let mut idx = None;
        let mut min_bt = i32::MAX;

        for i in 0..n {
            if !done[i] && procs[i].at <= time && procs[i].bt < min_bt {
                min_bt = procs[i].bt;
                idx = Some(i);
            }
        }

        match idx {
            None => time += 1,
            Some(i) => {
                time += procs[i].bt;
                procs[i].ct = time;
                procs[i].tat = procs[i].ct - procs[i].at;
                procs[i].wt = procs[i].tat - procs[i].bt;
                done[i] = true;
                completed += 1;
            }
        }
    }
    print_result(procs);
}

fn priority_scheduling(procs: &mut [Process]) {
    let n = procs.len();
    let mut completed = 0;
    let mut time = 0;
    let mut done = vec![false; n];

    while completed < n {
        let mut idx = None;
        let mut best_pr = i32::MAX;

        for i in 0..n {
            if !done[i] && procs[i].at <= time && procs[i].pr < best_pr {
                best_pr = procs[i].pr;
                idx = Some(i);
            }
        }

        match idx {
            None => time += 1,
            Some(i) => {
                time += procs[i].bt;
                procs[i].ct = time;
                procs[i].tat = procs[i].ct - procs[i].at;
                procs[i].wt = procs[i].tat - procs[i].bt;
                done[i] = true;
                completed += 1;
            }
        }
    }
    print_result(procs);
}

fn round_robin(procs: &mut [Process], quantum: i32) {
    let n = procs.len();
    let mut time = 0;
    let mut completed = 0;
    for p in procs.iter_mut() {
        p.rem = p.bt;
    }

    let mut queue = VecDeque::new();
    let mut in_queue = vec![false; n];

    for i in 0..n {
        if procs[i].at <= time {
            queue.push_back(i);
            in_queue[i] = true;
        }
    }

    while completed < n {
        if queue.is_empty() {
            time += 1;
            for i in 0..n {
                if !in_queue[i] && procs[i].rem > 0 && procs[i].at <= time {
                    queue.push_back(i);
                    in_queue[i] = true;
                }
            }
            continue;
        }

        let idx = queue.pop_front().unwrap();
        let slice = procs[idx].rem.min(quantum);
        time += slice;
        procs[idx].rem -= slice;

        for i in 0..n {
            if !in_queue[i] && procs[i].rem > 0 && procs[i].at <= time && i != idx {
                queue.push_back(i);
                in_queue[i] = true;
            }
        }

        if procs[idx].rem > 0 {
            queue.push_back(idx);
        } else {
            procs[idx].ct = time;
            procs[idx].tat = procs[idx].ct - procs[idx].at;
            procs[idx].wt = procs[idx].tat - procs[idx].bt;
            completed += 1;
            in_queue[idx] = false;
        }
    }
    print_result(procs);
}

fn main() {
    print!("Enter number of processes: ");
    io::stdout().flush().unwrap();
    let n: usize = read_line().trim().parse().unwrap();

    let mut procs = Vec::with_capacity(n);
    for i in 0..n {
        print!("Process {} - Arrival Time, Burst Time, Priority: ", i + 1);
        io::stdout().flush().unwrap();
        let nums: Vec<i32> = read_line()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        procs.push(Process {
            pid: i + 1,
            at: nums[0],
            bt: nums[1],
            pr: nums[2],
            ct: 0,
            tat: 0,
            wt: 0,
            rem: nums[1],
        });
    }

    println!("\nMenu:\n1. FCFS\n2. SJF (Non-preemptive)");
    println!("3. Priority (Non-preemptive)\n4. Round Robin");
    print!("Enter choice: ");
    io::stdout().flush().unwrap();
    let choice: i32 = read_line().trim().parse().unwrap();

    match choice {
        1 => fcfs(&mut procs),
        2 => sjf(&mut procs),
        3 => priority_scheduling(&mut procs),
        4 => {
            print!("Enter time quantum: ");
            io::stdout().flush().unwrap();
            let quantum: i32 = read_line().trim().parse().unwrap();
            round_robin(&mut procs, quantum);
        }
        _ => println!("Invalid choice"),
    }
}
