mod device;
mod nvme;
mod metrics;

use device::BlockDevice;
use nvme::{NvmeCommand, NvmeQueue};
use metrics::Metrics;

use tokio::sync::mpsc;


#[tokio::main]
async fn main() {
    // init block device of 512mb
    let device = BlockDevice::new(512 * 512 * 512);
    
    // init prometheus metrics
    let metrics = Metrics::new();

    // setup command channel
    let (cmd_tx, cmd_rx) = mpsc::channel(32);

    // start nvme queue worker
    let nvme_queue = NvmeQueue::new(device, cmd_rx, metrics.read_count.clone(), metrics.write_count.clone(), metrics.latency_histogram.clone());

    // start http server
    let metrics_route = metrics::Metrics::metrics_route();
    tokio::spawn(warp::serve(metrics_route).run(([0,0,0,0], 9090)));

    // send read command
    let (resp_tx, mut resp_rx) = mpsc::channel(1);
    cmd_tx.send(NvmeCommand::Read { offset: 0, length: 256, resp: resp_tx }).await.unwrap();

    if let Some(result) = resp_rx.recv().await {
        match result {
            Ok(data) => println!("Read {} bytes from NVMe device", data.len()),
            Err(e) => println!("Read error: {}", e),
        }
    }
}
