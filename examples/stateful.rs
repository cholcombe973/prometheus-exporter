extern crate prometheus_exporter;

use std::sync::{Arc, Mutex};
use prometheus_exporter::PrometheusExporter as Prometheus;
use prometheus_exporter::{PrometheusMetrics, Value};

struct PrometheusHistory {
    calls: Mutex<i64>,
}

impl PrometheusHistory {
    fn exporter_calls(&self) -> Value {
        if let Ok(mut calls) = self.calls.lock() {
            *calls += 1;
            Value::Integer(*calls)
        } else {
            Value::Integer(0)
        }
    }

    fn new() -> PrometheusHistory {
        PrometheusHistory {
            calls: Mutex::new(0),
        }
    }
}

fn main() {
    let history = PrometheusHistory::new();
    let mut metrics = PrometheusMetrics::new();
    // let
    // let arc = history.clone();
    metrics.add_metric("visits", move || history.exporter_calls());
    Prometheus::new(
        Arc::new(metrics)
    ).bind(9010);
}