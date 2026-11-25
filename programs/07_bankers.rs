use std::io::{self, Write};

const P: usize = 5;
const R: usize = 3;

fn read_line() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input
}

fn read_vec() -> Vec<i32> {
    read_line()
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect()
}

fn calculate_need(maximum: &[[i32; R]; P], allocation: &[[i32; R]; P]) -> [[i32; R]; P] {
    let mut need = [[0; R]; P];
    for i in 0..P {
        for j in 0..R {
            need[i][j] = maximum[i][j] - allocation[i][j];
        }
    }
    need
}

fn is_safe(
    available: &[i32; R],
    allocation: &[[i32; R]; P],
    need: &[[i32; R]; P],
) -> Option<[usize; P]> {
    let mut work = *available;
    let mut finish = [false; P];
    let mut safe_seq = [0; P];
    let mut count = 0;

    while count < P {
        let mut found = false;
        for i in 0..P {
            if !finish[i] && (0..R).all(|j| need[i][j] <= work[j]) {
                for k in 0..R {
                    work[k] += allocation[i][k];
                }
                safe_seq[count] = i;
                count += 1;
                finish[i] = true;
                found = true;
            }
        }
        if !found {
            return None;
        }
    }
    Some(safe_seq)
}

fn request_resources(
    pid: usize,
    request: &[i32; R],
    available: &mut [i32; R],
    allocation: &mut [[i32; R]; P],
    need: &mut [[i32; R]; P],
) -> bool {
    for j in 0..R {
        if request[j] > need[pid][j] {
            println!("Error: process has exceeded its maximum claim.");
            return false;
        }
        if request[j] > available[j] {
            println!("Request denied: not enough resources available.");
            return false;
        }
    }

    for j in 0..R {
        available[j] -= request[j];
        allocation[pid][j] += request[j];
        need[pid][j] -= request[j];
    }

    if is_safe(available, allocation, need).is_some() {
        println!("Request can be granted safely.");
        true
    } else {
        for j in 0..R {
            available[j] += request[j];
            allocation[pid][j] -= request[j];
            need[pid][j] += request[j];
        }
        println!("Request denied: would lead to an unsafe state.");
        false
    }
}

fn main() {
    print!("Enter available resources of each type ({} types): ", R);
    io::stdout().flush().unwrap();
    let avail_vec = read_vec();
    let mut available = [0; R];
    available.copy_from_slice(&avail_vec[..R]);

    println!("Enter Maximum matrix ({} processes x {} resources):", P, R);
    let mut maximum = [[0; R]; P];
    for i in 0..P {
        let row = read_vec();
        maximum[i].copy_from_slice(&row[..R]);
    }

    println!("Enter Allocation matrix:");
    let mut allocation = [[0; R]; P];
    for i in 0..P {
        let row = read_vec();
        allocation[i].copy_from_slice(&row[..R]);
    }

    let mut need = calculate_need(&maximum, &allocation);

    match is_safe(&available, &allocation, &need) {
        Some(safe_seq) => {
            println!("System is in a SAFE state.");
            print!("Safe sequence: ");
            for pid in safe_seq {
                print!("P{} ", pid);
            }
            println!();
        }
        None => println!("System is in an UNSAFE state."),
    }

    print!("\nEnter process number requesting resources: ");
    io::stdout().flush().unwrap();
    let pid: usize = read_line().trim().parse().unwrap();

    print!("Enter request vector ({} values): ", R);
    io::stdout().flush().unwrap();
    let req_vec = read_vec();
    let mut request = [0; R];
    request.copy_from_slice(&req_vec[..R]);

    request_resources(pid, &request, &mut available, &mut allocation, &mut need);
}
