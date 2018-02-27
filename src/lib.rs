extern crate hyper;
extern crate futures;
extern crate indexmap;

mod exporter;
mod prometheus_metric;

pub use exporter::PrometheusExporter;
pub use prometheus_metric::PrometheusMetric;
pub use prometheus_metric::PrometheusValue;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

