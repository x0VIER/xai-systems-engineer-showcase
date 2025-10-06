# Distributed Key-Value Store

<div align="center">
  <img src="https://github.com/x0VIER/xai-systems-engineer-showcase/raw/master/docs/distributed_kv_architecture.png" alt="Distributed KV Store Architecture" width="700">
</div>

<div align="center">
  <strong>A fault-tolerant, distributed key-value store built in Rust with consensus-based replication for high availability and data consistency.</strong>
</div>

<br />

<div align="center">
  <a href="https://github.com/x0VIER/xai-systems-engineer-showcase/blob/master/LICENSE">
    <img src="https://img.shields.io/badge/License-MIT-blue.svg" alt="License: MIT">
  </a>
  <a href="https://github.com/topics/distributed-systems">
    <img src="https://img.shields.io/badge/Type-Distributed_Systems-blueviolet" alt="Type: Distributed Systems">
  </a>
  <a href="https://github.com/topics/consensus">
    <img src="https://img.shields.io/badge/Algorithm-Raft_Consensus-orange" alt="Algorithm: Raft Consensus">
  </a>
  <a href="https://github.com/topics/grpc">
    <img src="https://img.shields.io/badge/Protocol-gRPC-green" alt="Protocol: gRPC">
  </a>
</div>

<br />

## Overview

This distributed key-value store provides a reliable and consistent data storage solution that can withstand node failures. It implements the Raft consensus algorithm to ensure that data remains consistent across all nodes in the cluster, even in the presence of network partitions or node failures.

## Key Features

- **Fault Tolerance:** Continues operating even when nodes fail
- **Strong Consistency:** Uses Raft consensus to ensure data consistency
- **High Availability:** Replicates data across multiple nodes
- **Efficient Communication:** Uses gRPC and Protocol Buffers for fast, type-safe communication
- **Interactive GUI:** Provides a simple interface for interacting with the store

## Architecture

The system consists of the following components:

1. **Cluster Nodes:** Multiple instances that form the distributed system
2. **Raft Consensus Module:** Handles leader election and log replication
3. **Storage Engine:** Manages the actual key-value data
4. **gRPC Server:** Provides the API for clients to interact with the store
5. **GUI Client:** A graphical interface for easy interaction with the store

## Implementation

### Protocol Definition

```protobuf
syntax = "proto3";
package distributed_kv;

service KvStore {
  rpc Get(GetRequest) returns (GetResponse);
  rpc Set(SetRequest) returns (SetResponse);
  rpc Delete(DeleteRequest) returns (DeleteResponse);
}

message GetRequest {
  string key = 1;
}

message GetResponse {
  string value = 1;
}

message SetRequest {
  string key = 1;
  string value = 2;
}

message SetResponse {
  bool success = 1;
}

message DeleteRequest {
  string key = 1;
}

message DeleteResponse {
  bool success = 1;
}
```

### Raft Store Implementation

```rust
pub struct RaftStore {
    id: u64,
    peers: Vec<u64>,
    state: RaftState,
    log: Vec<LogEntry>,
    commit_index: u64,
    last_applied: u64,
    // More fields...
}

impl RaftStore {
    pub fn new(id: u64, peers: Vec<u64>) -> Self {
        // Initialize the store...
    }

    pub fn get(&self, key: &str) -> Option<String> {
        // Get a value from the store...
    }

    pub fn set(&mut self, key: &str, value: &str) -> bool {
        // Set a value in the store...
    }

    pub fn delete(&mut self, key: &str) -> bool {
        // Delete a value from the store...
    }
}
```

## Technical Implementation

- **Consensus Protocol:** Raft algorithm for leader election and log replication
- **Communication:** gRPC with Protocol Buffers for efficient serialization
- **State Management:** In-memory state with log-based persistence
- **User Interface:** Built with egui/eframe for a native look and feel

## Usage

### Starting a Cluster Node

To start a node in the cluster:

```bash
./target/debug/distributed_kv
```

By default, the node will listen on port 50051 for client connections and use ports 50052+ for inter-node communication.

### Using the GUI Client

A graphical client is provided for easy interaction with the store:

```bash
cargo run --bin gui
```

This will open a window where you can:
- **Get:** Retrieve values by key
- **Set:** Store new key-value pairs
- **Delete:** Remove keys from the store

<div align="center">
  <img src="https://github.com/x0VIER/xai-systems-engineer-showcase/raw/master/docs/kv_store_gui.png" alt="KV Store GUI" width="500">
</div>

### API Operations

The store supports the following operations:

- **Get:** Retrieve a value by its key
- **Set:** Store a key-value pair
- **Delete:** Remove a key-value pair
- **List:** Enumerate all keys in the store

## Performance Considerations

The distributed nature of the system introduces some trade-offs:

- **Latency:** Write operations require consensus among a majority of nodes
- **Throughput:** Limited by the leader's capacity to process requests
- **Consistency vs. Availability:** The system prioritizes consistency over availability (CP in CAP theorem)

## Future Enhancements

- Add support for transactions across multiple keys
- Implement snapshotting for faster node recovery
- Add authentication and authorization
- Support for watch operations (notifications on key changes)
- Cluster membership changes without downtime
