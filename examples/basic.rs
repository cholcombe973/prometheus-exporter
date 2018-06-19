extern crate prometheus_exporter;

use prometheus_exporter::PrometheusExporterBuilder as Prometheus;
use prometheus_exporter::PrometheusMetric as Metric;
use prometheus_exporter::PrometheusValue as Value;
fn main() {
   Prometheus::new()
        .metric(
            Metric::new("cpu_usage").with_callback(|| Value::Float(cpu_usage()) )
        ).bind(9010);
}

fn cpu_usage() -> f64 {
    0.0
}