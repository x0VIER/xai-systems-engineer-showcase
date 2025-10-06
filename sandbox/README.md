# Secure Code Sandbox

<div align="center">
  <img src="https://github.com/x0VIER/xai-systems-engineer-showcase/raw/master/docs/sandbox_architecture.png" alt="Sandbox Architecture" width="700">
</div>

<div align="center">
  <strong>A robust, Rust-based sandbox for executing untrusted Python code in an isolated environment with comprehensive resource monitoring.</strong>
</div>

<br />

<div align="center">
  <a href="https://github.com/x0VIER/xai-systems-engineer-showcase/blob/master/LICENSE">
    <img src="https://img.shields.io/badge/License-MIT-blue.svg" alt="License: MIT">
  </a>
  <a href="https://github.com/topics/sandbox">
    <img src="https://img.shields.io/badge/Security-Isolation-green" alt="Security: Isolation">
  </a>
  <a href="https://github.com/topics/linux-namespaces">
    <img src="https://img.shields.io/badge/Feature-Linux_Namespaces-orange" alt="Feature: Linux Namespaces">
  </a>
</div>

<br />

## Overview

The Secure Code Sandbox provides a containerized execution environment for running untrusted Python scripts safely. It leverages `bubblewrap` to create strong isolation boundaries, preventing the executed code from accessing sensitive system resources or the network.

## Key Security Features

- **Process Isolation:** Runs in a new PID namespace, preventing visibility of or interaction with other system processes
- **Filesystem Isolation:** Provides read-only access to essential system directories (`/usr`, `/lib`, `/lib64`, `/bin`) and the script itself
- **Network Isolation:** Blocks all network access, preventing data exfiltration or command-and-control
- **Resource Monitoring:** Tracks CPU and memory usage in real-time, with visualization capabilities

## Architecture

The sandbox uses a multi-layered approach to security:

1. **Bubblewrap Container:** Creates a lightweight container with restricted capabilities
2. **Mount Namespace:** Provides a controlled view of the filesystem
3. **Resource Monitoring:** Tracks process resource usage via procfs
4. **Visualization:** Generates charts of resource utilization over time

## Implementation

```rust
use std::process::{Command, Stdio};
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

    // Monitor resource usage
    // ...
}
```

## Usage

### Running a Script

To execute a Python script in the sandbox:

```bash
./target/debug/secure_sandbox --script <path_to_script>
```

The sandbox will execute the script in an isolated environment and monitor its resource usage.

### Example Output

```
Python script running...
Network connection failed: [Errno -3] Temporary failure in name resolution
File write failed: [Errno 2] No such file or directory: '/etc/passwd'
Python script finished.
Python script exited with status: exit status: 0
```

### Resource Monitoring

The sandbox generates a `resource_usage.json` file containing CPU and memory usage data. To visualize this data:

```bash
python3 visualize.py
```

This will create a `resource_usage.png` file showing the resource usage over time:

<div align="center">
  <img src="https://github.com/x0VIER/xai-systems-engineer-showcase/raw/master/sandbox/resource_usage.png" alt="Resource Usage" width="600">
</div>

## Security Considerations

While this sandbox provides strong isolation, it is not intended for production use with truly malicious code. Additional hardening would be required, such as:

- Seccomp filters to restrict system calls
- More granular resource limits via cgroups
- Additional network isolation via network namespaces
- Time limits on execution

## Future Enhancements

- Add support for seccomp filters to restrict system calls
- Implement cgroups for more precise resource limiting
- Add support for running other interpreted languages
- Enhance visualization with interactive dashboards
