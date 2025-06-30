NVMe-like Virtual Block Device Simulator
A Rust-based simulator that emulates an NVMe-like virtual block device with async read/write capabilities and performance metrics.

Features
Simulates NVMe queues and commands (basic read/write).

Async command processing using Tokio.

Tracks I/O performance metrics (latency, IOPS).

Exposes Prometheus metrics endpoint (/metrics).
