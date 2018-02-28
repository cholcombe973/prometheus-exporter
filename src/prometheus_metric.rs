// use std::collections::HashMap;
use indexmap::IndexMap;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_builds_a_metric() {
        let expected = r#"haproxy_backend_bytes_in_total{backend="centauri.solutions"} 1824"#;
        let metric = PrometheusMetric::new("haproxy_backend_bytes_in_total").with_callback(|| PrometheusValue::Integer(1824) ).with_feature("backend", "centauri.solutions");
        assert_eq!(expected, metric.to_string());
    }

    #[test]
    fn it_builds_a_metric_with_multiple_features() {
        let expected = r#"node_cpu{cpu="cpu0",mode="user"} 1803.66"#;
        let metric = PrometheusMetric::new("node_cpu").with_callback(|| PrometheusValue::Float(1803.66) ).with_feature("cpu", "cpu0").with_feature("mode", "user");
        assert_eq!(expected, metric.to_string());
    }
}

pub struct PrometheusMetric {
    name: String,
    // value: PrometheusValue,
    callback: Option<Box<Fn() -> PrometheusValue>>,
    features: IndexMap<String, String>,
}

impl PrometheusMetric {
    pub fn new<T: Into<String>>(name: T) -> Self {
        PrometheusMetric {
            name: name.into(),
            callback: None,
            features: IndexMap::new(),
        }
    }

    pub fn with_callback<F: 'static>(mut self, callback: F) -> Self where
    // The closure takes no input and returns nothing.
    F: Fn() -> PrometheusValue {
        self.callback = Some(Box::new(callback));
        self
    }

    pub fn with_feature<T1: Into<String>, T2: Into<String>>(mut self, name: T1, value: T2) -> Self {
        self.features.insert(name.into(), value.into());
        self
    }

    pub fn to_string(&self) -> String {
        let mut s = self.name.clone();
        if let Some(ref callback) = self.callback {
            let mut features = String::new();
            let mut written = false;
            for (ref key, ref value) in &self.features {
                if written {
                    features.push_str(",");
                }
                features.push_str(&format!("{key}=\"{value}\"", key=key, value=value));
                written = true;
            }
            if written {
                s.push_str(&format!("{{{}}}", features));
            }
            s.push_str(" ");

            s.push_str( &callback().to_string() );
        }
        s
    }
}

#[derive(Clone, Debug)]
pub enum PrometheusValue {
    String(String),
    Float(f64),
    Integer(i64)
}

impl PrometheusValue {
    pub fn to_string(&self) -> String {
        match self {
            &PrometheusValue::String(ref s) => s.clone(),
            &PrometheusValue::Float(f) => format!("{}", f),
            &PrometheusValue::Integer(i) => format!("{}", i),
        }
    }
}
