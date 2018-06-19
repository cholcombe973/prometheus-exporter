extern crate prometheus_exporter;

use std::sync::{Arc, Mutex};
use prometheus_exporter::PrometheusExporterBuilder as Prometheus;
use prometheus_exporter::PrometheusMetric as Metric;
use prometheus_exporter::PrometheusValue as Value;

struct PrometheusHistory {
    calls: i64,
}

impl PrometheusHistory {
    fn exporter_calls(&mut self) -> Value {
        self.calls += 1;
        Value::Integer(self.calls)
    }
}

fn main() {
    let history = Arc::new(Mutex::new(PrometheusHistory{calls: 0}));
    Prometheus::new()
        .metric(
            Metric::new("exporter_calls").with_callback(move || {
                history.lock().unwrap().exporter_calls()
             } )
        ).bind(9010);
}
