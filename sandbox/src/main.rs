use std::process::{Command, Stdio, Child};
use std::time::{Duration, Instant};
use std::thread;
use clap::Parser;
use serde::{Serialize, Deserialize};
use procfs::process::Process;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    script: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ResourceUsage {
    timestamp: u64,
    cpu_usage: f64,
    memory_usage: u64,
}

fn main() {
    let args = Args::parse();

    let mut child = Command::new("bwrap")
        .arg("--unshare-all")
        .arg("--ro-bind")
        .arg("/usr")
        .arg("/usr")
        .arg("--ro-bind")
        .arg("/lib")
        .arg("/lib")
        .arg("--ro-bind")
        .arg("/lib64")
        .arg("/lib64")
        .arg("--ro-bind")
        .arg("/bin")
        .arg("/bin")
        .arg("--ro-bind")
        .arg(&args.script)
        .arg(&args.script)
        .arg("--proc")
        .arg("/proc")
        .arg("--dev")
        .arg("/dev")
        .arg("--")
        .arg("python3")
        .arg(&args.script)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .expect("Failed to execute command");

    let mut usage_data = Vec::new();
    let start_time = Instant::now();

    while let Ok(None) = child.try_wait() {
        if let Ok(process) = Process::new(child.id() as i32) {
            let cpu_usage = process.stat().unwrap().utime as f64 / procfs::ticks_per_second() as f64;
            let memory_usage = process.stat().unwrap().rss;
            usage_data.push(ResourceUsage {
                timestamp: start_time.elapsed().as_secs(),
                cpu_usage,
                memory_usage,
            });
        }
        thread::sleep(Duration::from_millis(100));
    }

    let status = child.wait().unwrap();

    let json_output = serde_json::to_string_pretty(&usage_data).unwrap();
    std::fs::write("resource_usage.json", json_output).unwrap();

    println!("\nPython script exited with status: {}", status);
}

