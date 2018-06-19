use std::slice::Iter;
use std::sync::{Arc, RwLock};

use PrometheusMetric;

pub trait MetricCollector: Clone {
    fn iter(&self) -> Iter<PrometheusMetric>;
}

#[derive(Clone)]
pub struct PrometheusMetricCollector {
    // stuff
    pub(crate) metrics: Arc<Vec<PrometheusMetric>>
}

impl MetricCollector for PrometheusMetricCollector {
    fn iter(&self) -> Iter<PrometheusMetric> {
        self.metrics.iter()
    }
}