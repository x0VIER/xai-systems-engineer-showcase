# Secure Code Sandbox

![Sandbox Architecture](../docs/sandbox_architecture.png)

A robust, Rust-based sandbox for executing untrusted Python code in an isolated environment with comprehensive resource monitoring.

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

![Resource Usage](../resource_usage.png)

## Implementation Details

The sandbox is implemented in Rust and uses the following components:

- **bubblewrap:** For process and filesystem isolation
- **procfs:** For monitoring process resource usage
- **clap:** For command-line argument parsing
- **serde:** For serializing resource usage data

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

