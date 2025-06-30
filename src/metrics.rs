use prometheus::{IntCounter, Histogram, Encoder, TextEncoder, register_int_counter, register_histogram};
use warp::Filter;

pub struct Metrics {
    pub read_count: IntCounter,
    pub write_count: IntCounter,
    pub latency_histogram: Histogram,
}

impl Metrics {
    pub fn new() -> Self {
        Self {
            read_count: register_int_counter!("nvme_read_ops_total", "Total NVMe read operations").unwrap(),
            write_count: register_int_counter!("nvme_write_ops_total", "Total NVMe write operations").unwrap(),
            latency_histogram: register_histogram!("nvme_cmd_latency_seconds", "Latency of NVMe commands").unwrap(),
        }
    }

    pub fn metrics_route() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path("metrics").map( || {
            let encoder = TextEncoder::new();
            let metrics_families = prometheus::gather();
            let mut buffer = Vec::new();
            encoder.encode(&metrics_families, &mut buffer).unwrap();

            warp::reply::with_header(
                buffer,
                "Content-Type",
                encoder.format_type(),
            )
        })
    }
}
