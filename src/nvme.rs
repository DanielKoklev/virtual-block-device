use crate::device::BlockDevice;
use tokio::sync::mpsc;
use std::time::{Instant};
use prometheus::{IntCounter, Histogram};

pub enum NvmeCommand {
    Read { offset: usize, length: usize, resp: mpsc::Sender<Result<Vec<u8>, String>>},
    Write { offset: usize, data: Vec<u8>, resp: mpsc::Sender<Result<(), String>>},
}

pub struct NvmeQueue {
    device: BlockDevice,
    cmd_rx: mpsc::Receiver<NvmeCommand>,
    // prometheus metrics
    read_count: IntCounter,
    write_count: IntCounter,
    latency_histogram: Histogram,
}

impl NvmeQueue {
    pub fn new(device: BlockDevice, cmd_rx: mpsc::Receiver<NvmeCommand>, read_count: IntCounter, write_count:IntCounter, latency_histogram: Histogram) -> Self {
        NvmeQueue { device, cmd_rx, read_count, write_count, latency_histogram}
    }

    pub async fn run(mut self) {
        while let Some(cmd) = self.cmd_rx.recv().await {
            let start = Instant::now();
            match cmd {
                NvmeCommand::Read { offset, length, resp } => {
                    let mut buf = vec![0u8; length];
                    let result = self.device.read(offset, &mut buf);
                    let _ = resp.send(result.map(|_| buf)).await;
                    self.read_count.inc();
                }
                NvmeCommand::Write { offset, data, resp } => {
                    let result = self.device.write(offset, &data);
                    let _ = resp.send(result).await;
                    self.write_count.inc();
                }
            }
            self.latency_histogram.observe(start.elapsed().as_secs_f64());
        }
    }
}
