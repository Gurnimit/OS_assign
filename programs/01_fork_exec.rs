use std::process::{Command, Stdio};

fn main() {
    let parent_pid = std::process::id();
    println!("Parent process: PID = {}", parent_pid);

    // Choose the directory listing command based on the OS
    let (program, args) = if cfg!(target_os = "windows") {
        ("cmd", vec!["/C", "dir"])
    } else {
        ("ls", vec!["-l"])
    };

    // Spawn a child process (equivalent to fork + exec)
    let mut child = Command::new(program)
        .args(&args)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .expect("Failed to spawn child process");

    let child_pid = child.id();
    println!("Parent waiting for child (PID = {}) to finish...", child_pid);
    println!(
        "Child process: PID = {}, Parent PID = {}",
        child_pid, parent_pid
    );
    println!("Child is executing '{} {}'", program, args.join(" "));

    // Wait for the child process to terminate and collect exit status
    let status = child.wait().expect("Failed to wait on child");

    if let Some(code) = status.code() {
        println!("Child exited with status {}", code);
    }
    println!("Parent process resumes and exits.");
}
