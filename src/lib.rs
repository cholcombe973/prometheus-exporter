extern crate hyper;
extern crate futures;
extern crate indexmap;

mod exporter;
mod metric;
mod metrics;


// pub use collector::{MetricCollector, PrometheusMetricCollector};
// pub use exporter::{PrometheusExporter, PrometheusExporterBuilder};
pub use exporter::PrometheusExporter;
pub use metric::{Metric, Value};
pub use metrics::{Metrics, PrometheusMetrics, BasicMetric};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

