use std::sync::Arc;

use Metric;
use Value;

pub trait Metrics {
    // type Item: Into<Metric>;
    fn metrics(&self) -> Vec<Metric>;
}

impl<T: Metrics> Metrics for Arc<T> {
    fn metrics(&self) -> Vec<Metric> {
        (**self).metrics()
    }
}

// #[derive(Clone)]
pub struct PrometheusMetrics {
    data: Vec<BasicMetric>,
}

impl PrometheusMetrics {
    pub fn new() -> PrometheusMetrics {
        PrometheusMetrics { data: vec![] }
    }
    pub fn add_metric<T: Into<String>, F: 'static>(&mut self, name: T, callback: F) where
    F: Fn() -> Value {
        self.data.push(BasicMetric {
            name: name.into(),
            value: Arc::new(Box::new(callback)),
        })
    }
}

// #[derive(Clone)]
pub struct BasicMetric {
    name: String,
    // value: Value,
    value: Arc<Box<Fn() -> Value>>,
}

impl BasicMetric {
    fn new<T: Into<String>, F: 'static>(name: T, callback: F) -> BasicMetric where
    F: Fn() -> Value{
        BasicMetric {
            name: name.into(),
            value: Arc::new(Box::new(callback)),
        }
    }
}

// impl <'a> Into<Metric> for &'a BasicMetric {
//     fn into(self) -> Metric {
//         Metric::new(self.name)
//             .with_value((self.value)())
//     }
// }

impl From<BasicMetric> for Metric {
    fn from(metric: BasicMetric) -> Metric {
        Metric::new(metric.name)
            .with_value( (metric.value)() )
    }
}

impl<'a> From<&'a BasicMetric> for Metric {
    fn from(metric: &'a BasicMetric) -> Metric {
        Metric::new(metric.name.clone())
            .with_value( (metric.value)() )
    }
}

// impl Into<Metric> for BasicMetric {
//     fn into(self) -> Metric {
//         Metric::new(self.name)
//             .with_value((self.value)())
//     }
// }

impl Metrics for PrometheusMetrics {
    // type Item = BasicMetric;
    fn metrics(&self) -> Vec<Metric> {
        self.data.iter().map(|a| a.into()).collect()
    }
}