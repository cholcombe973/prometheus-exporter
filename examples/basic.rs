extern crate prometheus_exporter;

// use prometheus_exporter::PrometheusExporterBuilder as Prometheus;
use prometheus_exporter::PrometheusExporter as Prometheus;
use prometheus_exporter::{PrometheusMetrics, Metric, Value};

fn main() {
    // Prometheus::new()
    //      .metric(
    //          Metric::new("cpu_usage").with_callback(|| Value::Float(cpu_usage()) )
    //      ).bind(9010);
    let mut metrics = PrometheusMetrics::new();
    metrics.add_metric("cpu_usage", || Value::Float(cpu_usage()));
    Prometheus::new(
        metrics
    ).bind(9010);
}

fn cpu_usage() -> f64 {
    0.0
}