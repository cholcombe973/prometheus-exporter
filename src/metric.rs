use std::collections::HashMap;
use indexmap::IndexMap;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_builds_a_metric() {
        let expected = r#"haproxy_backend_bytes_in_total{backend="centauri.solutions"} 1824"#;
        let metric = Metric::new("haproxy_backend_bytes_in_total").with_callback(|| Value::Integer(1824) ).with_feature("backend", "centauri.solutions");
        assert_eq!(expected, metric.to_string());
    }

    #[test]
    fn it_builds_a_metric_with_multiple_features() {
        let expected = r#"node_cpu{cpu="cpu0",mode="user"} 1803.66"#;
        let metric = Metric::new("node_cpu").with_callback(|| Value::Float(1803.66) ).with_feature("cpu", "cpu0").with_feature("mode", "user");
        assert_eq!(expected, metric.to_string());
    }
}

pub struct Metric {
    name: String,
    value: Option<Value>,
    callback: Option<Box<Fn() -> Value>>,
    features: IndexMap<String, String>,
}

impl Metric {
    pub fn new<T: Into<String>>(name: T) -> Self {
        Metric {
            name: name.into(),
            value: None,
            callback: None,
            features: IndexMap::new(),
        }
    }

    pub fn with_callback<F: 'static>(mut self, callback: F) -> Self where
    // The closure takes no input and returns a Value.
    F: Fn() -> Value {
        self.callback = Some(Box::new(callback));
        self.value = None;
        self
    }

    pub fn add_feature<T1: Into<String>, T2: Into<String>>(&mut self, name: T1, value: T2) {
        self.features.insert(name.into(), value.into());
    }

    pub fn add_features(&mut self, features: &HashMap<String,String>) {
        for (name, value) in features.iter(){
            self.add_feature(name.to_string(), value.to_string());
        }
    }

    pub fn add_value(&mut self, value: Value) {
        self.value = Some(value);
        self.callback = None;
    }

    pub fn with_value(mut self, value: Value) -> Self {
        self.value = Some(value);
        self.callback = None;
        self
    }

    pub fn with_feature<T1: Into<String>, T2: Into<String>>(mut self, name: T1, value: T2) -> Self {
        self.features.insert(name.into(), value.into());
        self
    }

    pub fn to_string(&self) -> String {
        let mut s = self.name.clone();
        if self.callback.is_some() || self.value.is_some() {
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
            if let Some(ref callback) = self.callback {
                s.push_str( &callback().to_string() );
            }
            if let Some(ref value) = self.value {
                s.push_str( &value.to_string() );
            }
        }
        s
    }
}

#[derive(Clone, Debug)]
pub enum Value {
    String(String),
    Float(f64),
    Integer(i64)
}

impl Value {
    pub fn to_string(&self) -> String {
        match self {
            &Value::String(ref s) => s.clone(),
            &Value::Float(f) => format!("{}", f),
            &Value::Integer(i) => format!("{}", i),
        }
    }
}
