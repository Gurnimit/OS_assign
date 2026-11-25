# Operating Systems Lab Practicals (Rust)

**Course:** CEC301 / CSC301 — Operating Systems  
**Program:** B.Tech / B.Sc. (Computer Science and Engineering) — Third Year  
**Language:** Rust (Standard Library)  

This repository contains Rust implementations for 11 core Operating Systems experiments covering process/thread management, CPU scheduling, process synchronization, deadlock avoidance, memory management, disk scheduling, and file allocation.

---

## List of Programs

| S.No. | Program Title | Source File |
| :---: | :--- | :--- |
| 1 | Introduction to System Calls — Process Creation (`spawn` / `fork`, `exec`, `wait`) | `programs/01_fork_exec.rs` |
| 2 | Thread Creation and Management (`std::thread`) | `programs/02_thread_sum.rs` |
| 3 | CPU Scheduling Algorithms — FCFS, SJF, Priority, and Round Robin | `programs/03_scheduling.rs` |
| 4 | Process Synchronization — Producer-Consumer Problem using Semaphores | `programs/04_producer_consumer.rs` |
| 5 | Process Synchronization — Readers-Writers Problem using Semaphores | `programs/05_readers_writers.rs` |
| 6 | Dining Philosophers Problem using Semaphores | `programs/06_dining_philosophers.rs` |
| 7 | Deadlock Avoidance using Banker's Algorithm | `programs/07_bankers.rs` |
| 8 | Contiguous Memory Allocation — First Fit, Best Fit, and Worst Fit | `programs/08_memory_allocation.rs` |
| 9 | Page Replacement Algorithms — FIFO, LRU, and Optimal | `programs/09_page_replacement.rs` |
| 10 | Disk Scheduling Algorithms — FCFS, SSTF, SCAN, and C-SCAN | `programs/10_disk_scheduling.rs` |
| 11 | File Allocation Methods — Contiguous, Linked, and Indexed Allocation | `programs/11_file_allocation.rs` |

---

## Prerequisites

- [Rust Toolchain](https://www.rust-lang.org/tools/install) (`rustc` and `cargo`, Rust 2021/2024 edition)
- Zero external crate dependencies — every program compiles using only the Rust standard library (`std`).

---

## How to Compile and Run

### Using `rustc` (Single File)

```bash
rustc programs/02_thread_sum.rs -o thread_sum
./thread_sum
```

*(On Windows Command Prompt, run `thread_sum.exe` instead of `./thread_sum`.)*

### Using `cargo`

Copy any `.rs` file into `src/main.rs` inside a Cargo project and run:

```bash
cargo run
```
